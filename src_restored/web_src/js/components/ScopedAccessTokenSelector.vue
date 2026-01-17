import { renderList as _renderList, Fragment as _Fragment, openBlock as _openBlock, createElementBlock as _createElementBlock, toDisplayString as _toDisplayString, createElementVNode as _createElementVNode } from "vue"

const _hoisted_1 = ["for"]
const _hoisted_2 = { class: "gitea-select" }
const _hoisted_3 = ["id"]
const _hoisted_4 = { value: "" }
const _hoisted_5 = ["value"]
const _hoisted_6 = ["value"]

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  return (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.categories, (category) => {
    return (_openBlock(), _createElementBlock("div", {
      key: category,
      class: "field tw-pl-1 tw-pb-1 access-token-category"
    }, [
      _createElementVNode("label", {
        class: "category-label",
        for: 'access-token-scope-' + category
      }, _toDisplayString(category), 9 /* TEXT, PROPS */, _hoisted_1),
      _createElementVNode("div", _hoisted_2, [
        _createElementVNode("select", {
          class: "ui selection access-token-select",
          name: "scope",
          id: 'access-token-scope-' + category
        }, [
          _createElementVNode("option", _hoisted_4, _toDisplayString(_ctx.noAccessLabel), 1 /* TEXT */),
          _createElementVNode("option", {
            value: 'read:' + category
          }, _toDisplayString(_ctx.readLabel), 9 /* TEXT, PROPS */, _hoisted_5),
          _createElementVNode("option", {
            value: 'write:' + category
          }, _toDisplayString(_ctx.writeLabel), 9 /* TEXT, PROPS */, _hoisted_6)
        ], 8 /* PROPS */, _hoisted_3)
      ])
    ]))
  }), 128 /* KEYED_FRAGMENT */))
}