import { toDisplayString as _toDisplayString, createElementVNode as _createElementVNode, resolveComponent as _resolveComponent, createVNode as _createVNode, Fragment as _Fragment, openBlock as _openBlock, createElementBlock as _createElementBlock } from "vue"

const _hoisted_1 = { class: "total-contributions" }

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_calendar_heatmap = _resolveComponent("calendar-heatmap")

  return (_openBlock(), _createElementBlock(_Fragment, null, [
    _createElementVNode("div", _hoisted_1, _toDisplayString($props.locale.contributions_in_the_last_12_months), 1 /* TEXT */),
    _createVNode(_component_calendar_heatmap, {
      locale: $props.locale,
      "no-data-text": $props.locale.contributions_zero,
      "tooltip-formatter": 
      (v) =>
        $props.locale.contributions_format
          .replace(
            '{contributions}',
            `<b>${v.count} ${
              v.count === 1
                ? $props.locale.contributions_one
                : $props.locale.contributions_few
            }</b>`
          )
          .replace('{month}', $props.locale.months[v.date.getMonth()])
          .replace('{day}', v.date.getDate())
          .replace('{year}', v.date.getFullYear())
    ,
      "end-date": _ctx.endDate,
      values: $props.values,
      "range-color": _ctx.colorRange,
      onDayClick: _cache[0] || (_cache[0] = $event => ($options.handleDayClick($event)))
    }, null, 8 /* PROPS */, ["locale", "no-data-text", "tooltip-formatter", "end-date", "values", "range-color"])
  ], 64 /* STABLE_FRAGMENT */))
}