import { createCommentVNode as _createCommentVNode, openBlock as _openBlock, createElementBlock as _createElementBlock, createElementVNode as _createElementVNode, vModelText as _vModelText, withDirectives as _withDirectives, toDisplayString as _toDisplayString, withModifiers as _withModifiers, Fragment as _Fragment, createTextVNode as _createTextVNode, normalizeClass as _normalizeClass, vModelCheckbox as _vModelCheckbox, resolveComponent as _resolveComponent, createVNode as _createVNode, renderList as _renderList } from "vue"

const _hoisted_1 = ["innerHTML"]
const _hoisted_2 = ["action"]
const _hoisted_3 = ["value"]
const _hoisted_4 = { class: "field" }
const _hoisted_5 = { class: "field" }
const _hoisted_6 = ["placeholder"]
const _hoisted_7 = ["data-tooltip-content"]
const _hoisted_8 = {
  key: 1,
  class: "field"
}
const _hoisted_9 = ["placeholder"]
const _hoisted_10 = ["value"]
const _hoisted_11 = {
  key: 2,
  class: "ui checkbox tw-ml-1"
}
const _hoisted_12 = { for: "delete-branch-after-merge" }
const _hoisted_13 = {
  key: 2,
  class: "tw-flex"
}
const _hoisted_14 = { class: "ui button" }
const _hoisted_15 = { class: "button-text" }
const _hoisted_16 = ["onClick"]
const _hoisted_17 = { class: "action-text" }
const _hoisted_18 = ["onClick"]
const _hoisted_19 = { class: "auto-merge-tip" }
const _hoisted_20 = ["onClick"]
const _hoisted_21 = { class: "action-text" }
const _hoisted_22 = ["action"]
const _hoisted_23 = ["value"]
const _hoisted_24 = { class: "ui button" }

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_svg_icon = _resolveComponent("svg-icon")

  return (_openBlock(), _createElementBlock(_Fragment, null, [
    _createCommentVNode("\n  if this component is shown, either the user is an admin (can do a merge without checks), or they are a writer who has the permission to do a merge\n  if the user is a writer and can't do a merge now (canMergeNow==false), then only show the Auto Merge for them\n  How to test the UI manually:\n  * Method 1: manually set some variables in pull.tmpl, eg: {{$notAllOverridableChecksOk = true}} {{$canMergeNow = false}}\n  * Method 2: make a protected branch, then set state=pending/success :\n    curl -X POST ${root_url}/api/v1/repos/${owner}/${repo}/statuses/${sha} \\\n      -H \"accept: application/json\" -H \"authorization: Basic $base64_auth\" -H \"Content-Type: application/json\" \\\n      -d '{\"context\": \"test/context\", \"description\": \"description\", \"state\": \"${state}\", \"target_url\": \"http://localhost\"}'\n  "),
    _createElementVNode("div", null, [
      _createCommentVNode(" eslint-disable-next-line vue/no-v-html "),
      (_ctx.mergeForm.hasPendingPullRequestMerge)
        ? (_openBlock(), _createElementBlock("div", {
            key: 0,
            innerHTML: _ctx.mergeForm.hasPendingPullRequestMergeTip,
            class: "ui info message"
          }, null, 8 /* PROPS */, _hoisted_1))
        : _createCommentVNode("v-if", true),
      _createCommentVNode(" another similar form is in pull.tmpl (manual merge)"),
      (_ctx.showActionForm)
        ? (_openBlock(), _createElementBlock("form", {
            key: 1,
            class: "ui form form-fetch-action",
            action: _ctx.mergeForm.baseLink+'/merge',
            method: "post"
          }, [
            _createElementVNode("input", {
              type: "hidden",
              name: "_csrf",
              value: _ctx.csrfToken
            }, null, 8 /* PROPS */, _hoisted_3),
            _withDirectives(_createElementVNode("input", {
              type: "hidden",
              name: "head_commit_id",
              "onUpdate:modelValue": _cache[0] || (_cache[0] = $event => ((_ctx.mergeForm.pullHeadCommitID) = $event))
            }, null, 512 /* NEED_PATCH */), [
              [_vModelText, _ctx.mergeForm.pullHeadCommitID]
            ]),
            _withDirectives(_createElementVNode("input", {
              type: "hidden",
              name: "merge_when_checks_succeed",
              "onUpdate:modelValue": _cache[1] || (_cache[1] = $event => ((_ctx.autoMergeWhenSucceed) = $event))
            }, null, 512 /* NEED_PATCH */), [
              [_vModelText, _ctx.autoMergeWhenSucceed]
            ]),
            _withDirectives(_createElementVNode("input", {
              type: "hidden",
              name: "force_merge",
              "onUpdate:modelValue": _cache[2] || (_cache[2] = $event => (($options.forceMerge) = $event))
            }, null, 512 /* NEED_PATCH */), [
              [_vModelText, $options.forceMerge]
            ]),
            (!_ctx.mergeStyleDetail.hideMergeMessageTexts)
              ? (_openBlock(), _createElementBlock(_Fragment, { key: 0 }, [
                  _createElementVNode("div", _hoisted_4, [
                    _withDirectives(_createElementVNode("input", {
                      type: "text",
                      name: "merge_title_field",
                      "onUpdate:modelValue": _cache[3] || (_cache[3] = $event => ((_ctx.mergeTitleFieldValue) = $event))
                    }, null, 512 /* NEED_PATCH */), [
                      [_vModelText, _ctx.mergeTitleFieldValue]
                    ])
                  ]),
                  _createElementVNode("div", _hoisted_5, [
                    _withDirectives(_createElementVNode("textarea", {
                      name: "merge_message_field",
                      rows: "5",
                      placeholder: _ctx.mergeForm.mergeMessageFieldPlaceHolder,
                      "onUpdate:modelValue": _cache[4] || (_cache[4] = $event => ((_ctx.mergeMessageFieldValue) = $event))
                    }, null, 8 /* PROPS */, _hoisted_6), [
                      [_vModelText, _ctx.mergeMessageFieldValue]
                    ]),
                    (_ctx.mergeMessageFieldValue !== _ctx.mergeForm.defaultMergeMessage)
                      ? (_openBlock(), _createElementBlock("button", {
                          key: 0,
                          onClick: _cache[5] || (_cache[5] = _withModifiers((...args) => ($options.clearMergeMessage && $options.clearMergeMessage(...args)), ["prevent"])),
                          class: "btn tw-mt-1 tw-p-1 interact-fg",
                          "data-tooltip-content": _ctx.mergeForm.textClearMergeMessageHint
                        }, _toDisplayString(_ctx.mergeForm.textClearMergeMessage), 9 /* TEXT, PROPS */, _hoisted_7))
                      : _createCommentVNode("v-if", true)
                  ])
                ], 64 /* STABLE_FRAGMENT */))
              : _createCommentVNode("v-if", true),
            (_ctx.mergeStyle === 'manually-merged')
              ? (_openBlock(), _createElementBlock("div", _hoisted_8, [
                  _createElementVNode("input", {
                    type: "text",
                    name: "merge_commit_id",
                    placeholder: _ctx.mergeForm.textMergeCommitId
                  }, null, 8 /* PROPS */, _hoisted_9)
                ]))
              : _createCommentVNode("v-if", true),
            _createElementVNode("button", {
              class: _normalizeClass(["ui button", $options.mergeButtonStyleClass]),
              type: "submit",
              name: "do",
              value: _ctx.mergeStyle
            }, [
              _createTextVNode(_toDisplayString(_ctx.mergeStyleDetail.textDoMerge) + " ", 1 /* TEXT */),
              (_ctx.autoMergeWhenSucceed)
                ? (_openBlock(), _createElementBlock(_Fragment, { key: 0 }, [
                    _createTextVNode(_toDisplayString(_ctx.mergeForm.textAutoMergeButtonWhenSucceed), 1 /* TEXT */)
                  ], 64 /* STABLE_FRAGMENT */))
                : _createCommentVNode("v-if", true)
            ], 10 /* CLASS, PROPS */, _hoisted_10),
            _createElementVNode("button", {
              class: "ui button merge-cancel",
              onClick: _cache[6] || (_cache[6] = $event => ($options.toggleActionForm(false)))
            }, _toDisplayString(_ctx.mergeForm.textCancel), 1 /* TEXT */),
            (_ctx.mergeForm.isPullBranchDeletable)
              ? (_openBlock(), _createElementBlock("div", _hoisted_11, [
                  _withDirectives(_createElementVNode("input", {
                    name: "delete_branch_after_merge",
                    type: "checkbox",
                    "onUpdate:modelValue": _cache[7] || (_cache[7] = $event => ((_ctx.deleteBranchAfterMerge) = $event)),
                    id: "delete-branch-after-merge"
                  }, null, 512 /* NEED_PATCH */), [
                    [_vModelCheckbox, _ctx.deleteBranchAfterMerge]
                  ]),
                  _createElementVNode("label", _hoisted_12, _toDisplayString(_ctx.mergeForm.textDeleteBranch), 1 /* TEXT */)
                ]))
              : _createCommentVNode("v-if", true)
          ], 8 /* PROPS */, _hoisted_2))
        : _createCommentVNode("v-if", true),
      (!_ctx.showActionForm)
        ? (_openBlock(), _createElementBlock("div", _hoisted_13, [
            _createCommentVNode(" the merge button "),
            _createElementVNode("div", {
              class: _normalizeClass(["ui buttons merge-button", [_ctx.mergeForm.emptyCommit ? 'grey' : _ctx.mergeForm.allOverridableChecksOk ? 'primary' : 'red']]),
              onClick: _cache[9] || (_cache[9] = $event => ($options.toggleActionForm(true)))
            }, [
              _createElementVNode("button", _hoisted_14, [
                _createVNode(_component_svg_icon, { name: "octicon-git-merge" }),
                _createElementVNode("span", _hoisted_15, [
                  _createTextVNode(_toDisplayString(_ctx.mergeStyleDetail.textDoMerge) + " ", 1 /* TEXT */),
                  (_ctx.autoMergeWhenSucceed)
                    ? (_openBlock(), _createElementBlock(_Fragment, { key: 0 }, [
                        _createTextVNode(_toDisplayString(_ctx.mergeForm.textAutoMergeButtonWhenSucceed), 1 /* TEXT */)
                      ], 64 /* STABLE_FRAGMENT */))
                    : _createCommentVNode("v-if", true)
                ])
              ]),
              (_ctx.mergeStyleAllowedCount>1)
                ? (_openBlock(), _createElementBlock("div", {
                    key: 0,
                    class: "ui dropdown icon button",
                    onClick: _cache[8] || (_cache[8] = _withModifiers($event => (_ctx.showMergeStyleMenu = !_ctx.showMergeStyleMenu), ["stop"]))
                  }, [
                    _createVNode(_component_svg_icon, {
                      name: "octicon-triangle-down",
                      size: 14
                    }),
                    _createElementVNode("div", {
                      class: _normalizeClass(["menu", {'show':_ctx.showMergeStyleMenu}])
                    }, [
                      (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.mergeForm.mergeStyles, (msd) => {
                        return (_openBlock(), _createElementBlock(_Fragment, null, [
                          _createCommentVNode(" if can merge now, show one action \"merge now\", and an action \"auto merge when succeed\" "),
                          (msd.allowed && _ctx.mergeForm.canMergeNow)
                            ? (_openBlock(), _createElementBlock("div", {
                                class: "item",
                                key: msd.name,
                                onClick: _withModifiers($event => ($options.switchMergeStyle(msd.name)), ["stop"])
                              }, [
                                _createElementVNode("div", _hoisted_17, _toDisplayString(msd.textDoMerge), 1 /* TEXT */),
                                (!msd.hideAutoMerge)
                                  ? (_openBlock(), _createElementBlock("div", {
                                      key: 0,
                                      class: "auto-merge-small",
                                      onClick: _withModifiers($event => ($options.switchMergeStyle(msd.name, true)), ["stop"])
                                    }, [
                                      _createVNode(_component_svg_icon, {
                                        name: "octicon-clock",
                                        size: 14
                                      }),
                                      _createElementVNode("div", _hoisted_19, _toDisplayString(_ctx.mergeForm.textAutoMergeWhenSucceed), 1 /* TEXT */)
                                    ], 8 /* PROPS */, _hoisted_18))
                                  : _createCommentVNode("v-if", true)
                              ], 8 /* PROPS */, _hoisted_16))
                            : _createCommentVNode("v-if", true),
                          _createCommentVNode(" if can NOT merge now, only show one action \"auto merge when succeed\" "),
                          (msd.allowed && !_ctx.mergeForm.canMergeNow && !msd.hideAutoMerge)
                            ? (_openBlock(), _createElementBlock("div", {
                                class: "item",
                                key: msd.name,
                                onClick: _withModifiers($event => ($options.switchMergeStyle(msd.name, true)), ["stop"])
                              }, [
                                _createElementVNode("div", _hoisted_21, _toDisplayString(msd.textDoMerge) + " " + _toDisplayString(_ctx.mergeForm.textAutoMergeButtonWhenSucceed), 1 /* TEXT */)
                              ], 8 /* PROPS */, _hoisted_20))
                            : _createCommentVNode("v-if", true)
                        ], 64 /* STABLE_FRAGMENT */))
                      }), 256 /* UNKEYED_FRAGMENT */))
                    ], 2 /* CLASS */)
                  ]))
                : _createCommentVNode("v-if", true)
            ], 2 /* CLASS */),
            _createCommentVNode(" the cancel auto merge button "),
            (_ctx.mergeForm.hasPendingPullRequestMerge)
              ? (_openBlock(), _createElementBlock("form", {
                  key: 0,
                  action: _ctx.mergeForm.baseLink+'/cancel_auto_merge',
                  method: "post",
                  class: "tw-ml-4"
                }, [
                  _createElementVNode("input", {
                    type: "hidden",
                    name: "_csrf",
                    value: _ctx.csrfToken
                  }, null, 8 /* PROPS */, _hoisted_23),
                  _createElementVNode("button", _hoisted_24, _toDisplayString(_ctx.mergeForm.textAutoMergeCancelSchedule), 1 /* TEXT */)
                ], 8 /* PROPS */, _hoisted_22))
              : _createCommentVNode("v-if", true)
          ]))
        : _createCommentVNode("v-if", true)
    ])
  ], 2112 /* STABLE_FRAGMENT, DEV_ROOT_FRAGMENT */))
}