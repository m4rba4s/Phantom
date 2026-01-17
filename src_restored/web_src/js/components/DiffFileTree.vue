import { createCommentVNode as _createCommentVNode, renderList as _renderList, Fragment as _Fragment, openBlock as _openBlock, createElementBlock as _createElementBlock, resolveComponent as _resolveComponent, createBlock as _createBlock, toDisplayString as _toDisplayString, withModifiers as _withModifiers, normalizeClass as _normalizeClass, createElementVNode as _createElementVNode } from "vue"

const _hoisted_1 = {
  key: 0,
  class: "diff-file-tree-items"
}
const _hoisted_2 = {
  key: 0,
  class: "tw-pt-1"
}

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_DiffFileTreeItem = _resolveComponent("DiffFileTreeItem")

  return (_ctx.store.fileTreeIsVisible)
    ? (_openBlock(), _createElementBlock("div", _hoisted_1, [
        _createCommentVNode(" only render the tree if we're visible. in many cases this is something that doesn't change very often "),
        (_openBlock(true), _createElementBlock(_Fragment, null, _renderList($options.fileTree, (item) => {
          return (_openBlock(), _createBlock(_component_DiffFileTreeItem, {
            key: item.name,
            item: item
          }, null, 8 /* PROPS */, ["item"]))
        }), 128 /* KEYED_FRAGMENT */)),
        (_ctx.store.isIncomplete)
          ? (_openBlock(), _createElementBlock("div", _hoisted_2, [
              _createElementVNode("a", {
                class: _normalizeClass(['ui', 'basic', 'tiny', 'button', _ctx.store.isLoadingNewData ? 'disabled' : '']),
                onClick: _cache[0] || (_cache[0] = _withModifiers((...args) => ($options.loadMoreData && $options.loadMoreData(...args)), ["stop"]))
              }, _toDisplayString(_ctx.store.showMoreMessage), 3 /* TEXT, CLASS */)
            ]))
          : _createCommentVNode("v-if", true)
      ]))
    : _createCommentVNode("v-if", true)
}