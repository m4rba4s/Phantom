import { resolveComponent as _resolveComponent, createVNode as _createVNode, withModifiers as _withModifiers, createElementVNode as _createElementVNode, openBlock as _openBlock, createElementBlock as _createElementBlock, createCommentVNode as _createCommentVNode, toDisplayString as _toDisplayString, withKeys as _withKeys, normalizeClass as _normalizeClass, renderList as _renderList, Fragment as _Fragment, createTextVNode as _createTextVNode, vShow as _vShow, withDirectives as _withDirectives } from "vue"

const _hoisted_1 = { class: "ui scrolling dropdown custom" }
const _hoisted_2 = ["data-tooltip-content", "aria-label"]
const _hoisted_3 = ["aria-expanded"]
const _hoisted_4 = {
  key: 0,
  class: "loading-indicator is-loading"
}
const _hoisted_5 = { class: "gt-ellipsis" }
const _hoisted_6 = { class: "gt-ellipsis text light-2 tw-mb-0" }
const _hoisted_7 = { class: "gt-ellipsis" }
const _hoisted_8 = { class: "gt-ellipsis text light-2" }
const _hoisted_9 = {
  key: 3,
  class: "info text light-2"
}
const _hoisted_10 = ["onKeydown", "onMouseover", "onClick"]
const _hoisted_11 = { class: "tw-flex-1 tw-flex tw-flex-col tw-gap-1" }
const _hoisted_12 = { class: "gt-ellipsis commit-list-summary" }
const _hoisted_13 = { class: "gt-ellipsis text light-2" }
const _hoisted_14 = { class: "text right" }
const _hoisted_15 = ["datetime"]
const _hoisted_16 = { class: "tw-font-mono" }

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_svg_icon = _resolveComponent("svg-icon")

  return (_openBlock(), _createElementBlock("div", _hoisted_1, [
    _createElementVNode("button", {
      class: "ui basic button",
      id: "diff-commit-list-expand",
      onClick: _cache[0] || (_cache[0] = _withModifiers($event => ($options.toggleMenu()), ["stop"])),
      "data-tooltip-content": _ctx.locale.filter_changes_by_commit,
      "aria-haspopup": "true",
      "aria-controls": "diff-commit-selector-menu",
      "aria-label": _ctx.locale.filter_changes_by_commit,
      "aria-activedescendant": "diff-commit-list-show-all"
    }, [
      _createVNode(_component_svg_icon, { name: "octicon-git-commit" })
    ], 8 /* PROPS */, _hoisted_2),
    _withDirectives(_createElementVNode("div", {
      class: _normalizeClass(["menu left transition", {visible: _ctx.menuVisible}]),
      id: "diff-commit-selector-menu",
      "aria-expanded": _ctx.menuVisible ? 'true': 'false'
    }, [
      (_ctx.isLoading)
        ? (_openBlock(), _createElementBlock("div", _hoisted_4))
        : _createCommentVNode("v-if", true),
      (!_ctx.isLoading)
        ? (_openBlock(), _createElementBlock("div", {
            key: 1,
            class: "vertical item",
            id: "diff-commit-list-show-all",
            role: "menuitem",
            onKeydown: _cache[1] || (_cache[1] = _withKeys($event => ($options.showAllChanges()), ["enter"])),
            onClick: _cache[2] || (_cache[2] = $event => ($options.showAllChanges()))
          }, [
            _createElementVNode("div", _hoisted_5, _toDisplayString(_ctx.locale.show_all_commits), 1 /* TEXT */),
            _createElementVNode("div", _hoisted_6, _toDisplayString(_ctx.locale.stats_num_commits), 1 /* TEXT */)
          ], 32 /* NEED_HYDRATION */))
        : _createCommentVNode("v-if", true),
      _createCommentVNode(" only show the show changes since last review if there is a review AND we are commits ahead of the last review "),
      (_ctx.lastReviewCommitSha != null)
        ? (_openBlock(), _createElementBlock("div", {
            key: 2,
            role: "menuitem",
            class: _normalizeClass(["vertical item", {disabled: !$options.commitsSinceLastReview}]),
            onKeydown: _cache[3] || (_cache[3] = _withKeys($event => ($options.changesSinceLastReviewClick()), ["enter"])),
            onClick: _cache[4] || (_cache[4] = $event => ($options.changesSinceLastReviewClick()))
          }, [
            _createElementVNode("div", _hoisted_7, _toDisplayString(_ctx.locale.show_changes_since_your_last_review), 1 /* TEXT */),
            _createElementVNode("div", _hoisted_8, _toDisplayString($options.commitsSinceLastReview) + " commits ", 1 /* TEXT */)
          ], 34 /* CLASS, NEED_HYDRATION */))
        : _createCommentVNode("v-if", true),
      (!_ctx.isLoading)
        ? (_openBlock(), _createElementBlock("span", _hoisted_9, _toDisplayString(_ctx.locale.select_commit_hold_shift_for_range), 1 /* TEXT */))
        : _createCommentVNode("v-if", true),
      (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.commits, (commit) => {
        return (_openBlock(), _createElementBlock("div", {
          key: commit.id,
          class: _normalizeClass(["vertical item", {selection: commit.selected, hovered: commit.hovered}]),
          role: "menuitem",
          onKeydown: [
            _withKeys(_withModifiers($event => ($options.commitClicked(commit.id)), ["exact"]), ["enter"]),
            _withKeys(_withModifiers($event => ($options.commitClickedShift(commit)), ["shift","exact"]), ["enter"])
          ],
          onMouseover: _withModifiers($event => ($options.highlight(commit)), ["shift"]),
          onClick: [
            _withModifiers($event => ($options.commitClicked(commit.id)), ["exact"]),
            _withModifiers($event => ($options.commitClicked(commit.id, true)), ["ctrl","exact"]),
            _withModifiers($event => ($options.commitClicked(commit.id, true)), ["meta","exact"]),
            _withModifiers($event => ($options.commitClickedShift(commit)), ["shift","exact","stop","prevent"])
          ]
        }, [
          _createElementVNode("div", _hoisted_11, [
            _createElementVNode("div", _hoisted_12, _toDisplayString(commit.summary), 1 /* TEXT */),
            _createElementVNode("div", _hoisted_13, [
              _createTextVNode(_toDisplayString(commit.committer_or_author_name) + " ", 1 /* TEXT */),
              _createElementVNode("span", _hoisted_14, [
                _createCommentVNode(" TODO: make this respect the PreferredTimestampTense setting "),
                _createElementVNode("relative-time", {
                  prefix: "",
                  datetime: commit.time,
                  "data-tooltip-content": "",
                  "data-tooltip-interactive": "true"
                }, _toDisplayString(commit.time), 9 /* TEXT, PROPS */, _hoisted_15)
              ])
            ])
          ]),
          _createElementVNode("div", _hoisted_16, _toDisplayString(commit.short_sha), 1 /* TEXT */)
        ], 42 /* CLASS, PROPS, NEED_HYDRATION */, _hoisted_10))
      }), 128 /* KEYED_FRAGMENT */))
    ], 10 /* CLASS, PROPS */, _hoisted_3), [
      [_vShow, _ctx.menuVisible]
    ])
  ]))
}