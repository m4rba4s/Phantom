import { resolveComponent as _resolveComponent, normalizeClass as _normalizeClass, openBlock as _openBlock, createBlock as _createBlock, createCommentVNode as _createCommentVNode, createElementBlock as _createElementBlock } from "vue"

const _hoisted_1 = ["data-tooltip-content"]

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_SvgIcon = _resolveComponent("SvgIcon")

  return ($props.status)
    ? (_openBlock(), _createElementBlock("span", {
        key: 0,
        class: "tw-flex tw-items-center",
        "data-tooltip-content": $props.localeStatus ?? $props.status
      }, [
        ($props.status === 'success')
          ? (_openBlock(), _createBlock(_component_SvgIcon, {
              key: 0,
              name: "octicon-check-circle-fill",
              class: _normalizeClass(["text green", $props.className]),
              size: $props.size
            }, null, 8 /* PROPS */, ["size", "class"]))
          : ($props.status === 'skipped')
            ? (_openBlock(), _createBlock(_component_SvgIcon, {
                key: 1,
                name: "octicon-skip",
                class: _normalizeClass(["text grey", $props.className]),
                size: $props.size
              }, null, 8 /* PROPS */, ["size", "class"]))
            : ($props.status === 'cancelled')
              ? (_openBlock(), _createBlock(_component_SvgIcon, {
                  key: 2,
                  name: "octicon-stop",
                  class: _normalizeClass(["text yellow", $props.className]),
                  size: $props.size
                }, null, 8 /* PROPS */, ["size", "class"]))
              : ($props.status === 'waiting')
                ? (_openBlock(), _createBlock(_component_SvgIcon, {
                    key: 3,
                    name: "octicon-clock",
                    class: _normalizeClass(["text yellow", $props.className]),
                    size: $props.size
                  }, null, 8 /* PROPS */, ["size", "class"]))
                : ($props.status === 'blocked')
                  ? (_openBlock(), _createBlock(_component_SvgIcon, {
                      key: 4,
                      name: "octicon-blocked",
                      class: _normalizeClass(["text yellow", $props.className]),
                      size: $props.size
                    }, null, 8 /* PROPS */, ["size", "class"]))
                  : ($props.status === 'running')
                    ? (_openBlock(), _createBlock(_component_SvgIcon, {
                        key: 5,
                        name: "octicon-meter",
                        class: _normalizeClass(["text yellow", 'job-status-rotate ' + $props.className]),
                        size: $props.size
                      }, null, 8 /* PROPS */, ["size", "class"]))
                    : (_openBlock(), _createBlock(_component_SvgIcon, {
                        key: 6,
                        name: "octicon-x-circle-fill",
                        class: "text red",
                        size: $props.size
                      }, null, 8 /* PROPS */, ["size"])),
        _createCommentVNode(" failure, unknown ")
      ], 8 /* PROPS */, _hoisted_1))
    : _createCommentVNode("v-if", true)
}