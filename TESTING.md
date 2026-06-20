# Phantom: eBPF & TUN Testing Guide

Данное руководство описывает, как провести сквозное тестирование (End-to-End) добавленных компонентов: виртуального сетевого интерфейса (TUN) и паттерн-детектора на базе eBPF (XDP).

## 1. Сборка eBPF (XDP) программы

eBPF требует компиляции под специализированный таргет (`bpfel-unknown-none`). Для этого потребуется `nightly` версия компилятора и исходники ядра (`build-std`).

```bash
cd netprobe-ebpf
cargo +nightly build --release --target bpfel-unknown-none -Z build-std=core,compiler_builtins
cd ..
```

После этого скомпилированный байткод будет доступен по пути:
`netprobe-ebpf/target/bpfel-unknown-none/release/netprobe-ebpf`

## 2. Тестирование паттерн-фильтра XDP (Магический байт)

XDP программа проверяет UDP пакеты. Если полезная нагрузка UDP начинается с `0xDEADBEEF` — пакет отбрасывается (`XDP_DROP`).

### Шаг 2.1: Создание тестового интерфейса

Создадим dummy-интерфейс (или veth-пару) для безопасного тестирования на уровне ядра:

```bash
sudo ip link add dev xdp_test type dummy
sudo ip link set up dev xdp_test
sudo ip addr add 10.10.10.1/24 dev xdp_test
```

### Шаг 2.2: Прикрепление eBPF программы через кастомный загрузчик

Для `aya` мы используем встроенную команду `ebpf-filter`.

Запустите фильтр в первом окне терминала:

```bash
sudo target/release/phantom --i-am-authorized ebpf-filter --iface xdp_test
```

**Отправка пакета:**
В другом окне терминала отправляем обычный пакет (пройдёт):
```bash
python3 -c "import socket; socket.socket(socket.AF_INET, socket.SOCK_DGRAM).sendto(b'HELLO_WORLD', ('10.10.10.1', 9999))"
```

Отправляем пакет с магическим паттерном `0xDEADBEEF` (будет заблокирован ядром на этапе XDP):
```bash
python3 -c "import socket; socket.socket(socket.AF_INET, socket.SOCK_DGRAM).sendto(b'\xDE\xAD\xBE\xEF_HACK', ('10.10.10.1', 9999))"
```

**Ожидаемый результат:** 
Пакет с `HELLO_WORLD` достигнет приложения (можно проверить через `tcpdump -i xdp_test -n udp`), а пакет с `\xDE\xAD\xBE\xEF` будет молча дропнут XDP, и не появится в `tcpdump` (т.к. XDP отрабатывает до `tcpdump`).

## 3. Тестирование TUN моста (TunBridge)

TUN интерфейс требует права `CAP_NET_ADMIN`. 

Для тестирования моста (`TunBridge`):

1. Убедитесь, что находитесь в **корневой директории** проекта `Phantom`, а не внутри `netprobe-ebpf`:
   ```bash
   cd ~/luke/Phantom  # или просто cd .. если вы были в netprobe-ebpf
   cargo build --release
   sudo setcap cap_net_admin=eip target/release/phantom
   ```

2. При запуске, Phantom создаст интерфейс `phantom0`.
3. Убедитесь, что интерфейс создан:
   ```bash
   ip a show dev phantom0
   ```
4. Вы можете настроить маршрутизацию через этот TUN интерфейс и запустить `iperf3` или `ping` для проверки пропускной способности. Мост корректно обернет данные (например, в QUIC/WireGuard) и передаст их наружу через сокет (и наоборот).

## 4. Очистка среды

```bash
sudo ip link del xdp_test
```
