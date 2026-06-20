# PHANTOM Architecture & Internals

## 1. System Overview

**Phantom** is a high-performance, stealth-focused traffic masquerading framework built in Rust. It is designed exclusively for authorized Red/Purple team operations. Its primary goal is to emulate Advanced Persistent Threat (APT) network behavior, enabling security teams to validate the efficacy of Intrusion Detection Systems (IDS), Data Loss Prevention (DLP) solutions, and Next-Generation Firewalls (NGFW).

By leveraging deep packet manipulation, protocol mimicry, and timing obfuscation, Phantom allows operators to execute reconnaissance and exfiltration scenarios that blend seamlessly into legitimate network noise.

## 2. Core Subsystems

Архитектура проекта разделена на независимые, но слабо связанные модули, каждый из которых отвечает за свой слой маскировки.

### 2.1. Scanner (`src/scanner/`)
Обеспечивает скрытое сетевое сканирование (Stealth SYN).
*   **Packet Crafter:** Собирает сырые (raw) IP/TCP пакеты, позволяя выставлять кастомные флаги.
*   **Fragmenter:** Дробит заголовки TCP на крошечные IP-фрагменты (Tiny Fragmentation) и поддерживает перекрывающиеся смещения (Overlapping Fragments) для обхода сигнатурных анализаторов.
*   **Decoys:** Генерирует математически корректные фальшивые IP-адреса, подмешивая их в трафик, чтобы скрыть реальный IP-адрес атакующего.

### 2.2. Mimicry (`src/mimicry/`)
Отвечает за обход L7-анализаторов (WAF, DPI).
*   **TLS/JA3 Spoofing:** Подменяет отпечатки Client Hello на уровне байтов, эмулируя легитимные браузеры (Google Chrome, Safari, Firefox).
*   **HTTP Header Manipulation:** Принудительно меняет порядок, регистр и структуру HTTP-заголовков в точном соответствии с профилем эмулируемого клиента.

### 2.3. Proxy & Wrap (`src/proxy/`)
Интерфейс интеграции со сторонними инструментами.
*   Перехватывает трафик локальных утилит (например, `curl`, `nmap` или кастомных эксплойтов).
*   Пропускает трафик через движок `Mimicry`, ротирует исходные порты (Source Port) и на лету модифицирует полезную нагрузку.

### 2.4. Tunneling (`src/tunnel/`)
Реализует скрытые каналы связи (Covert Channels) для эксфильтрации данных и C2-коммуникаций.
*   **DNS Tunneling:** Инкапсулирует данные в TXT-записи, разбивая их на чанки (Base32/Base64).
*   **ICMP Tunneling:** Встраивает данные в поле `payload` ICMP Echo-запросов.
*   **DoH (DNS over HTTPS):** Скрывает DNS-запросы внутри легитимного HTTPS-трафика к публичным резолверам (Google, Cloudflare).

### 2.5. Timing & Jitter (`src/timing/`)
Защита от обнаружения по паттернам времени (AI/ML эвристики).
*   **Jitter Generator:** Вносит случайные, нелинейные задержки между отправкой пакетов, эмулируя асинхронное поведение человека или легитимных фоновых процессов.

---

## 3. Threat Model & Security Constraints (Правила игры)

Phantom разрабатывался с учетом жестких ограничений (RoE - Rules of Engagement):
1.  **Strict Authorization Gate:** Утилита откажется работать без явного подтверждения авторизации (флаг `--i-am-authorized` или чекбокс в интерактивном меню).
2.  **No Exploit Payloads:** Фреймворк занимается *только* доставкой и маскировкой трафика. Он не содержит 0-day уязвимостей или вредоносного кода.
3.  **Local-First Testing:** По умолчанию DNS-резолвинг отключен во избежание утечек. Оператор обязан явно передавать IP-адреса.

---

## 4. Production Hardening Roadmap (Улучшения до уровня APT)

Для перевода инструмента из стадии "лабораторного proof-of-concept" в production-grade решение для Red Team, запланированы следующие архитектурные изменения:

### 4.1. Переход с Raw Sockets на eBPF / XDP
*   **Риск текущей архитектуры:** Сырые сокеты легко детектируются современными EDR (Sysmon, CrowdStrike), работают в user-space и требуют привилегий `root` / `CAP_NET_RAW`.
*   **Решение:** Перенос логики генерации пакетов и фрагментации на уровень **eBPF (Extended Berkeley Packet Filter) / XDP (eXpress Data Path)**. Это позволит модифицировать трафик прямо в ядре сети до его попадания в сетевой стек ОС, обеспечивая максимальную невидимость для user-space мониторинга и нулевой overhead по производительности.

### 4.2. Dynamic Fingerprint Generation (JA4 + HTTP/2 Parity)
*   **Риск текущей архитектуры:** Статические JA3-хэши быстро устаревают. Современные WAF коррелируют JA3 с HTTP/2 фреймами (Settings, Window Update). Несоответствие TLS-отпечатка и HTTP/2 паттерна приводит к мгновенной блокировке.
*   **Решение:** Внедрение стандарта **JA4**. Реализация "Live Feed" парсера: утилита будет в фоне подключаться к публичным сервисам настоящим браузером (через headless-режим или заранее собранную базу), извлекать актуальные слепки TLS + HTTP/2 и динамически натягивать их на исходящий трафик.

### 4.3. Strict Rate-Limiting & Concurrency Control (Token Bucket)
*   **Риск текущей архитектуры:** Неконтролируемая асинхронность `tokio` при сканировании тысяч портов может привести к исчерпанию файловых дескрипторов (Self-DoS) и резкому всплеску трафика (Micro-bursts), что является триггером для NGFW.
*   **Решение:** Внедрение алгоритма **Token Bucket** (использование крейта `governor`). Жесткое лимитирование Bandwidth (байт/сек) и Packet Rate (пакетов/сек). Внедрение `tokio::sync::Semaphore` для ограничения пула одновременно открытых сокетов, чтобы трафик всегда оставался ниже пороговых значений сенсоров IDS.
