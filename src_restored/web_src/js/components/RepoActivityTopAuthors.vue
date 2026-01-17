import { createElementVNode as _createElementVNode, resolveComponent as _resolveComponent, createVNode as _createVNode, openBlock as _openBlock, createElementBlock as _createElementBlock } from "vue"

const _hoisted_1 = {
  class: "activity-bar-graph",
  ref: "style",
  style: {"width":"0","height":"0"}
}

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_Bar = _resolveComponent("Bar")

  return (_openBlock(), _createElementBlock("div", null, [
    _createElementVNode("div", _hoisted_1, null, 512 /* NEED_PATCH */),
    _createVNode(_component_Bar, {
      height: "150px",
      data: _ctx.graphPoints(),
      options: _ctx.getOptions()
    }, null, 8 /* PROPS */, ["data", "options"])
  ]))
}