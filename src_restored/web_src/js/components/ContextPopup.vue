import { openBlock as _openBlock, createElementBlock as _createElementBlock, createCommentVNode as _createCommentVNode, toDisplayString as _toDisplayString, createElementVNode as _createElementVNode, resolveComponent as _resolveComponent, normalizeClass as _normalizeClass, createVNode as _createVNode, createTextVNode as _createTextVNode, renderList as _renderList, Fragment as _Fragment, normalizeStyle as _normalizeStyle } from "vue"

const _hoisted_1 = { ref: "root" }
const _hoisted_2 = {
  key: 0,
  class: "tw-h-12 tw-w-12 is-loading"
}
const _hoisted_3 = {
  key: 1,
  id: "issue-info-popup"
}
const _hoisted_4 = { class: "labels-list" }
const _hoisted_5 = ["innerHTML"]
const _hoisted_6 = { key: 2 }

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_svg_icon = _resolveComponent("svg-icon")

  return (_openBlock(), _createElementBlock("div", _hoisted_1, [
    (_ctx.loading)
      ? (_openBlock(), _createElementBlock("div", _hoisted_2))
      : _createCommentVNode("v-if", true),
    (!_ctx.loading && _ctx.issue !== null)
      ? (_openBlock(), _createElementBlock("div", _hoisted_3, [
          _createElementVNode("p", null, [
            _createElementVNode("small", null, _toDisplayString(_ctx.issue.repository.full_name) + " on " + _toDisplayString($options.createdAt), 1 /* TEXT */)
          ]),
          _createElementVNode("p", null, [
            _createVNode(_component_svg_icon, {
              name: $options.icon,
              class: _normalizeClass(['text', $options.color])
            }, null, 8 /* PROPS */, ["name", "class"]),
            _cache[0] || (_cache[0] = _createTextVNode()),
            _createElementVNode("strong", null, _toDisplayString(_ctx.issue.title), 1 /* TEXT */),
            _createTextVNode(" #" + _toDisplayString(_ctx.issue.number), 1 /* TEXT */)
          ]),
          _createElementVNode("p", null, _toDisplayString($options.body), 1 /* TEXT */),
          _createElementVNode("div", _hoisted_4, [
            _createCommentVNode(" eslint-disable-next-line vue/no-v-html "),
            (_openBlock(true), _createElementBlock(_Fragment, null, _renderList($options.labels, (label) => {
              return (_openBlock(), _createElementBlock("div", {
                key: label.name,
                class: "ui label",
                style: _normalizeStyle({ color: label.textColor, backgroundColor: label.color }),
                innerHTML: label.name
              }, null, 12 /* STYLE, PROPS */, _hoisted_5))
            }), 128 /* KEYED_FRAGMENT */))
          ])
        ]))
      : _createCommentVNode("v-if", true),
    (!_ctx.loading && _ctx.issue === null)
      ? (_openBlock(), _createElementBlock("div", _hoisted_6, [
          _createElementVNode("p", null, [
            _createElementVNode("small", null, _toDisplayString(_ctx.i18nErrorOccurred), 1 /* TEXT */)
          ]),
          _createElementVNode("p", null, _toDisplayString(_ctx.i18nErrorMessage), 1 /* TEXT */)
        ]))
      : _createCommentVNode("v-if", true)
  ], 512 /* NEED_PATCH */))
}