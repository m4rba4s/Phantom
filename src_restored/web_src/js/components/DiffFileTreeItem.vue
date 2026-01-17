import { createCommentVNode as _createCommentVNode, resolveComponent as _resolveComponent, createVNode as _createVNode, toDisplayString as _toDisplayString, createElementVNode as _createElementVNode, normalizeClass as _normalizeClass, openBlock as _openBlock, createElementBlock as _createElementBlock, withModifiers as _withModifiers, renderList as _renderList, Fragment as _Fragment, createBlock as _createBlock, vShow as _vShow, withDirectives as _withDirectives } from "vue"

const _hoisted_1 = ["title", "href"]
const _hoisted_2 = { class: "gt-ellipsis tw-flex-1" }
const _hoisted_3 = ["title"]
const _hoisted_4 = { class: "gt-ellipsis" }
const _hoisted_5 = {
  key: 2,
  class: "sub-items"
}

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_SvgIcon = _resolveComponent("SvgIcon")
  const _component_DiffFileTreeItem = _resolveComponent("DiffFileTreeItem", true)

  return (_openBlock(), _createElementBlock(_Fragment, null, [
    _createCommentVNode("title instead of tooltip above as the tooltip needs too much work with the current methods, i.e. not being loaded or staying open for \"too long\""),
    ($props.item.isFile)
      ? (_openBlock(), _createElementBlock("a", {
          key: 0,
          class: _normalizeClass(["item-file", {'selected': _ctx.store.selectedItem === '#diff-' + $props.item.file.NameHash, 'viewed': $props.item.file.IsViewed}]),
          title: $props.item.name,
          href: '#diff-' + $props.item.file.NameHash
        }, [
          _createCommentVNode(" file "),
          _createVNode(_component_SvgIcon, { name: "octicon-file" }),
          _createElementVNode("span", _hoisted_2, _toDisplayString($props.item.name), 1 /* TEXT */),
          _createVNode(_component_SvgIcon, {
            name: $options.getIconForDiffType($props.item.file.Type).name,
            class: _normalizeClass($options.getIconForDiffType($props.item.file.Type).classes)
          }, null, 8 /* PROPS */, ["name", "class"])
        ], 10 /* CLASS, PROPS */, _hoisted_1))
      : (_openBlock(), _createElementBlock("div", {
          key: 1,
          class: "item-directory",
          title: $props.item.name,
          onClick: _cache[0] || (_cache[0] = _withModifiers($event => (_ctx.collapsed = !_ctx.collapsed), ["stop"]))
        }, [
          _createCommentVNode(" directory "),
          _createVNode(_component_SvgIcon, {
            name: _ctx.collapsed ? 'octicon-chevron-right' : 'octicon-chevron-down'
          }, null, 8 /* PROPS */, ["name"]),
          _createVNode(_component_SvgIcon, {
            class: "text primary",
            name: "octicon-file-directory-fill"
          }),
          _createElementVNode("span", _hoisted_4, _toDisplayString($props.item.name), 1 /* TEXT */)
        ], 8 /* PROPS */, _hoisted_3)),
    ($props.item.children?.length)
      ? _withDirectives((_openBlock(), _createElementBlock("div", _hoisted_5, [
          (_openBlock(true), _createElementBlock(_Fragment, null, _renderList($props.item.children, (childItem) => {
            return (_openBlock(), _createBlock(_component_DiffFileTreeItem, {
              key: childItem.name,
              item: childItem
            }, null, 8 /* PROPS */, ["item"]))
          }), 128 /* KEYED_FRAGMENT */))
        ], 512 /* NEED_PATCH */)), [
          [_vShow, !_ctx.collapsed]
        ])
      : _createCommentVNode("v-if", true)
  ], 64 /* STABLE_FRAGMENT */))
}