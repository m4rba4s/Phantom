import { toDisplayString as _toDisplayString, createTextVNode as _createTextVNode, Fragment as _Fragment, openBlock as _openBlock, createElementBlock as _createElementBlock, createCommentVNode as _createCommentVNode, resolveComponent as _resolveComponent, createBlock as _createBlock, createElementVNode as _createElementVNode, createVNode as _createVNode, withKeys as _withKeys, vModelText as _vModelText, withDirectives as _withDirectives, normalizeClass as _normalizeClass, renderList as _renderList, vShow as _vShow, withModifiers as _withModifiers } from "vue"

const _hoisted_1 = { class: "ui dropdown custom" }
const _hoisted_2 = { class: "text tw-flex tw-items-center tw-mr-1 gt-ellipsis" }
const _hoisted_3 = { class: "ui icon search input" }
const _hoisted_4 = { class: "icon" }
const _hoisted_5 = ["placeholder"]
const _hoisted_6 = {
  key: 0,
  class: "branch-tag-tab"
}
const _hoisted_7 = {
  class: "scrolling menu",
  ref: "scrollContainer"
}
const _hoisted_8 = {
  key: 0,
  class: "loading-indicator is-loading"
}
const _hoisted_9 = ["onClick"]
const _hoisted_10 = {
  key: 0,
  class: "ui label"
}
const _hoisted_11 = ["href"]
const _hoisted_12 = ["textContent"]
const _hoisted_13 = ["textContent"]
const _hoisted_14 = { class: "text small" }
const _hoisted_15 = { key: 0 }
const _hoisted_16 = { key: 1 }
const _hoisted_17 = { key: 2 }
const _hoisted_18 = ["action"]
const _hoisted_19 = ["value"]
const _hoisted_20 = {
  key: 1,
  class: "message"
}

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_svg_icon = _resolveComponent("svg-icon")

  return (_openBlock(), _createElementBlock("div", _hoisted_1, [
    _createElementVNode("button", {
      class: "branch-dropdown-button gt-ellipsis ui basic small compact button tw-flex tw-m-0",
      onClick: _cache[0] || (_cache[0] = $event => (_ctx.menuVisible = !_ctx.menuVisible)),
      onKeyup: _cache[1] || (_cache[1] = _withKeys($event => (_ctx.menuVisible = !_ctx.menuVisible), ["enter"]))
    }, [
      _createElementVNode("span", _hoisted_2, [
        (_ctx.release)
          ? (_openBlock(), _createElementBlock(_Fragment, { key: 0 }, [
              _createTextVNode(_toDisplayString(_ctx.textReleaseCompare), 1 /* TEXT */)
            ], 64 /* STABLE_FRAGMENT */))
          : (_openBlock(), _createElementBlock(_Fragment, { key: 1 }, [
              (_ctx.isViewTag)
                ? (_openBlock(), _createBlock(_component_svg_icon, {
                    key: 0,
                    name: "octicon-tag"
                  }))
                : (_ctx.isViewBranch)
                  ? (_openBlock(), _createBlock(_component_svg_icon, {
                      key: 1,
                      name: "octicon-git-branch"
                    }))
                  : (_openBlock(), _createBlock(_component_svg_icon, {
                      key: 2,
                      name: "octicon-git-commit"
                    })),
              _createElementVNode("strong", {
                ref: "dropdownRefName",
                class: "tw-ml-2 tw-inline-block gt-ellipsis"
              }, _toDisplayString(_ctx.refNameText), 513 /* TEXT, NEED_PATCH */)
            ], 64 /* STABLE_FRAGMENT */))
      ]),
      _createVNode(_component_svg_icon, {
        name: "octicon-triangle-down",
        size: 14,
        class: "dropdown icon"
      })
    ], 32 /* NEED_HYDRATION */),
    _withDirectives(_createElementVNode("div", {
      class: _normalizeClass(["menu transition", {visible: _ctx.menuVisible}])
    }, [
      _createElementVNode("div", _hoisted_3, [
        _createElementVNode("i", _hoisted_4, [
          _createVNode(_component_svg_icon, {
            name: "octicon-filter",
            size: 16
          })
        ]),
        _withDirectives(_createElementVNode("input", {
          name: "search",
          ref: "searchField",
          autocomplete: "off",
          "onUpdate:modelValue": _cache[2] || (_cache[2] = $event => ((_ctx.searchTerm) = $event)),
          onKeydown: _cache[3] || (_cache[3] = $event => (_ctx.keydown($event))),
          placeholder: _ctx.searchFieldPlaceholder
        }, null, 40 /* PROPS, NEED_HYDRATION */, _hoisted_5), [
          [_vModelText, _ctx.searchTerm]
        ])
      ]),
      (_ctx.showBranchesInDropdown)
        ? (_openBlock(), _createElementBlock("div", _hoisted_6, [
            _createElementVNode("a", {
              class: _normalizeClass(["branch-tag-item muted", {active: _ctx.mode === 'branches'}]),
              href: "#",
              onClick: _cache[4] || (_cache[4] = $event => (_ctx.handleTabSwitch('branches')))
            }, [
              _createVNode(_component_svg_icon, {
                name: "octicon-git-branch",
                size: 16
              }),
              _createTextVNode(_toDisplayString(_ctx.textBranches), 1 /* TEXT */)
            ], 2 /* CLASS */),
            (!_ctx.noTag)
              ? (_openBlock(), _createElementBlock("a", {
                  key: 0,
                  class: _normalizeClass(["branch-tag-item muted", {active: _ctx.mode === 'tags'}]),
                  href: "#",
                  onClick: _cache[5] || (_cache[5] = $event => (_ctx.handleTabSwitch('tags')))
                }, [
                  _createVNode(_component_svg_icon, {
                    name: "octicon-tag",
                    size: 16
                  }),
                  _createTextVNode(_toDisplayString(_ctx.textTags), 1 /* TEXT */)
                ], 2 /* CLASS */))
              : _createCommentVNode("v-if", true)
          ]))
        : _createCommentVNode("v-if", true),
      _cache[13] || (_cache[13] = _createElementVNode("div", { class: "branch-tag-divider" }, null, -1 /* HOISTED */)),
      _createElementVNode("div", _hoisted_7, [
        _createVNode(_component_svg_icon, {
          name: "octicon-rss",
          "symbol-id": "svg-symbol-octicon-rss"
        }),
        (_ctx.isLoading)
          ? (_openBlock(), _createElementBlock("div", _hoisted_8))
          : _createCommentVNode("v-if", true),
        (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.filteredItems, (item, index) => {
          return (_openBlock(), _createElementBlock("div", {
            key: item.name,
            class: _normalizeClass(["item", {selected: item.selected, active: _ctx.active === index}]),
            onClick: $event => (_ctx.selectItem(item)),
            ref_for: true,
            ref: 'listItem' + index
          }, [
            _createTextVNode(_toDisplayString(item.name) + " ", 1 /* TEXT */),
            (item.name===_ctx.repoDefaultBranch && _ctx.mode === 'branches')
              ? (_openBlock(), _createElementBlock("div", _hoisted_10, _toDisplayString(_ctx.textDefaultBranchLabel), 1 /* TEXT */))
              : _createCommentVNode("v-if", true),
            _withDirectives(_createElementVNode("a", {
              role: "button",
              class: "rss-icon tw-float-right",
              href: _ctx.rssURLPrefix + item.url,
              target: "_blank",
              onClick: _cache[6] || (_cache[6] = _withModifiers(() => {}, ["stop"]))
            }, [
              _createCommentVNode(" creating a lot of Vue component is pretty slow, so we use a static SVG here "),
              _cache[11] || (_cache[11] = _createElementVNode("svg", {
                width: "14",
                height: "14",
                class: "svg octicon-rss"
              }, [
                _createElementVNode("use", { href: "#svg-symbol-octicon-rss" })
              ], -1 /* HOISTED */))
            ], 8 /* PROPS */, _hoisted_11), [
              [_vShow, _ctx.enableFeed && _ctx.mode === 'branches']
            ])
          ], 10 /* CLASS, PROPS */, _hoisted_9))
        }), 128 /* KEYED_FRAGMENT */)),
        (_ctx.showCreateNewBranch)
          ? (_openBlock(), _createElementBlock("div", {
              key: 1,
              class: _normalizeClass(["item", {active: _ctx.active === _ctx.filteredItems.length}]),
              ref: 'listItem' + _ctx.filteredItems.length
            }, [
              _createElementVNode("a", {
                href: "#",
                onClick: _cache[7] || (_cache[7] = $event => (_ctx.createNewBranch()))
              }, [
                _withDirectives(_createElementVNode("div", null, [
                  _cache[12] || (_cache[12] = _createElementVNode("i", { class: "reference tags icon" }, null, -1 /* HOISTED */)),
                  _createElementVNode("span", {
                    textContent: _toDisplayString(_ctx.textCreateTag.replace('%s', _ctx.searchTerm))
                  }, null, 8 /* PROPS */, _hoisted_12)
                ], 512 /* NEED_PATCH */), [
                  [_vShow, _ctx.shouldCreateTag]
                ]),
                _withDirectives(_createElementVNode("div", null, [
                  _createVNode(_component_svg_icon, { name: "octicon-git-branch" }),
                  _createElementVNode("span", {
                    textContent: _toDisplayString(_ctx.textCreateBranch.replace('%s', _ctx.searchTerm))
                  }, null, 8 /* PROPS */, _hoisted_13)
                ], 512 /* NEED_PATCH */), [
                  [_vShow, !_ctx.shouldCreateTag]
                ]),
                _createElementVNode("div", _hoisted_14, [
                  (_ctx.isViewBranch || _ctx.release)
                    ? (_openBlock(), _createElementBlock("span", _hoisted_15, _toDisplayString(_ctx.textCreateBranchFrom.replace('%s', _ctx.branchName)), 1 /* TEXT */))
                    : (_ctx.isViewTag)
                      ? (_openBlock(), _createElementBlock("span", _hoisted_16, _toDisplayString(_ctx.textCreateBranchFrom.replace('%s', _ctx.tagName)), 1 /* TEXT */))
                      : (_openBlock(), _createElementBlock("span", _hoisted_17, _toDisplayString(_ctx.textCreateBranchFrom.replace('%s', _ctx.commitIdShort)), 1 /* TEXT */))
                ])
              ]),
              _createElementVNode("form", {
                ref: "newBranchForm",
                action: _ctx.formActionUrl,
                method: "post"
              }, [
                _createElementVNode("input", {
                  type: "hidden",
                  name: "_csrf",
                  value: _ctx.csrfToken
                }, null, 8 /* PROPS */, _hoisted_19),
                _withDirectives(_createElementVNode("input", {
                  type: "hidden",
                  name: "new_branch_name",
                  "onUpdate:modelValue": _cache[8] || (_cache[8] = $event => ((_ctx.searchTerm) = $event))
                }, null, 512 /* NEED_PATCH */), [
                  [_vModelText, _ctx.searchTerm]
                ]),
                _withDirectives(_createElementVNode("input", {
                  type: "hidden",
                  name: "create_tag",
                  "onUpdate:modelValue": _cache[9] || (_cache[9] = $event => ((_ctx.shouldCreateTag) = $event))
                }, null, 512 /* NEED_PATCH */), [
                  [_vModelText, _ctx.shouldCreateTag]
                ]),
                (_ctx.treePath)
                  ? _withDirectives((_openBlock(), _createElementBlock("input", {
                      key: 0,
                      type: "hidden",
                      name: "current_path",
                      "onUpdate:modelValue": _cache[10] || (_cache[10] = $event => ((_ctx.treePath) = $event))
                    }, null, 512 /* NEED_PATCH */)), [
                      [_vModelText, _ctx.treePath]
                    ])
                  : _createCommentVNode("v-if", true)
              ], 8 /* PROPS */, _hoisted_18)
            ], 2 /* CLASS */))
          : _createCommentVNode("v-if", true)
      ], 512 /* NEED_PATCH */),
      (_ctx.showNoResults && !_ctx.isLoading)
        ? (_openBlock(), _createElementBlock("div", _hoisted_20, _toDisplayString(_ctx.noResults), 1 /* TEXT */))
        : _createCommentVNode("v-if", true)
    ], 2 /* CLASS */), [
      [_vShow, _ctx.menuVisible]
    ])
  ]))
}