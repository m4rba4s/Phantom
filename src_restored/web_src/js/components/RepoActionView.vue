import { resolveComponent as _resolveComponent, createVNode as _createVNode, createCommentVNode as _createCommentVNode, createElementVNode as _createElementVNode, toDisplayString as _toDisplayString, openBlock as _openBlock, createElementBlock as _createElementBlock, createTextVNode as _createTextVNode, renderList as _renderList, Fragment as _Fragment, createBlock as _createBlock, normalizeClass as _normalizeClass, withKeys as _withKeys, withModifiers as _withModifiers, vShow as _vShow, withDirectives as _withDirectives } from "vue"

const _hoisted_1 = { class: "ui container action-view-container" }
const _hoisted_2 = { class: "action-view-header" }
const _hoisted_3 = { class: "action-info-summary" }
const _hoisted_4 = { class: "action-info-summary-title" }
const _hoisted_5 = ["innerHTML"]
const _hoisted_6 = ["data-url"]
const _hoisted_7 = { class: "action-summary" }
const _hoisted_8 = ["href"]
const _hoisted_9 = ["href"]
const _hoisted_10 = {
  key: 0,
  class: "ui label tw-max-w-full"
}
const _hoisted_11 = ["data-tooltip-content"]
const _hoisted_12 = ["href", "data-tooltip-content"]
const _hoisted_13 = { class: "action-summary" }
const _hoisted_14 = ["href"]
const _hoisted_15 = { class: "action-view-body" }
const _hoisted_16 = { class: "action-view-left" }
const _hoisted_17 = { class: "job-group-section" }
const _hoisted_18 = { class: "job-brief-list" }
const _hoisted_19 = ["href"]
const _hoisted_20 = { class: "job-brief-item-left" }
const _hoisted_21 = { class: "job-brief-name tw-mx-2 gt-ellipsis" }
const _hoisted_22 = { class: "job-brief-item-right" }
const _hoisted_23 = { class: "step-summary-duration" }
const _hoisted_24 = {
  key: 0,
  class: "job-artifacts"
}
const _hoisted_25 = { class: "job-artifacts-title" }
const _hoisted_26 = { class: "job-artifacts-list" }
const _hoisted_27 = ["href"]
const _hoisted_28 = ["onClick"]
const _hoisted_29 = { class: "action-view-right" }
const _hoisted_30 = { class: "job-info-header" }
const _hoisted_31 = { class: "job-info-header-left gt-ellipsis" }
const _hoisted_32 = { class: "job-info-header-title gt-ellipsis" }
const _hoisted_33 = { class: "job-info-header-detail" }
const _hoisted_34 = { class: "job-info-header-right" }
const _hoisted_35 = { class: "btn gt-interact-bg tw-p-2" }
const _hoisted_36 = { class: "icon" }
const _hoisted_37 = { class: "icon" }
const _hoisted_38 = { class: "icon" }
const _hoisted_39 = ["href"]
const _hoisted_40 = { class: "icon" }
const _hoisted_41 = {
  key: 0,
  class: "job-step-container",
  ref: "steps"
}
const _hoisted_42 = ["onClick", "onKeyup"]
const _hoisted_43 = { class: "step-summary-msg gt-ellipsis" }
const _hoisted_44 = { class: "step-summary-duration" }

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_ActionRunStatus = _resolveComponent("ActionRunStatus")
  const _component_SvgIcon = _resolveComponent("SvgIcon")

  return (_openBlock(), _createElementBlock("div", _hoisted_1, [
    _createElementVNode("div", _hoisted_2, [
      _createElementVNode("div", _hoisted_3, [
        _createElementVNode("div", _hoisted_4, [
          _createVNode(_component_ActionRunStatus, {
            "locale-status": _ctx.locale.status[_ctx.run.status],
            status: _ctx.run.status,
            size: 20
          }, null, 8 /* PROPS */, ["locale-status", "status"]),
          _createCommentVNode(" eslint-disable-next-line vue/no-v-html "),
          _createElementVNode("h2", {
            class: "action-info-summary-title-text",
            innerHTML: _ctx.run.titleHTML
          }, null, 8 /* PROPS */, _hoisted_5)
        ]),
        (_ctx.run.canApprove)
          ? (_openBlock(), _createElementBlock("button", {
              key: 0,
              class: "ui basic small compact button primary",
              onClick: _cache[0] || (_cache[0] = $event => (_ctx.approveRun()))
            }, _toDisplayString(_ctx.locale.approve), 1 /* TEXT */))
          : (_ctx.run.canCancel)
            ? (_openBlock(), _createElementBlock("button", {
                key: 1,
                class: "ui basic small compact button red",
                onClick: _cache[1] || (_cache[1] = $event => (_ctx.cancelRun()))
              }, _toDisplayString(_ctx.locale.cancel), 1 /* TEXT */))
            : (_ctx.run.canRerun)
              ? (_openBlock(), _createElementBlock("button", {
                  key: 2,
                  class: "ui basic small compact button tw-mr-0 tw-whitespace-nowrap link-action",
                  "data-url": `${_ctx.run.link}/rerun`
                }, _toDisplayString(_ctx.locale.rerun_all), 9 /* TEXT, PROPS */, _hoisted_6))
              : _createCommentVNode("v-if", true)
      ]),
      _createElementVNode("div", _hoisted_7, [
        _createTextVNode(_toDisplayString(_ctx.run.commit.localeCommit) + " ", 1 /* TEXT */),
        _createElementVNode("a", {
          class: "muted",
          href: _ctx.run.commit.link
        }, _toDisplayString(_ctx.run.commit.shortSHA), 9 /* TEXT, PROPS */, _hoisted_8),
        _createTextVNode(" " + _toDisplayString(_ctx.run.commit.localePushedBy) + " ", 1 /* TEXT */),
        _createElementVNode("a", {
          class: "muted",
          href: _ctx.run.commit.pusher.link
        }, _toDisplayString(_ctx.run.commit.pusher.displayName), 9 /* TEXT, PROPS */, _hoisted_9),
        (_ctx.run.commit.shortSHA)
          ? (_openBlock(), _createElementBlock("span", _hoisted_10, [
              (_ctx.run.commit.branch.isDeleted)
                ? (_openBlock(), _createElementBlock("span", {
                    key: 0,
                    class: "gt-ellipsis tw-line-through",
                    "data-tooltip-content": _ctx.run.commit.branch.name
                  }, _toDisplayString(_ctx.run.commit.branch.name), 9 /* TEXT, PROPS */, _hoisted_11))
                : (_openBlock(), _createElementBlock("a", {
                    key: 1,
                    class: "gt-ellipsis",
                    href: _ctx.run.commit.branch.link,
                    "data-tooltip-content": _ctx.run.commit.branch.name
                  }, _toDisplayString(_ctx.run.commit.branch.name), 9 /* TEXT, PROPS */, _hoisted_12))
            ]))
          : _createCommentVNode("v-if", true)
      ]),
      _createElementVNode("div", _hoisted_13, [
        _createTextVNode(_toDisplayString(_ctx.run.commit.localeWorkflow) + " ", 1 /* TEXT */),
        _createElementVNode("a", {
          class: "muted",
          href: _ctx.workflowURL
        }, _toDisplayString(_ctx.workflowName), 9 /* TEXT, PROPS */, _hoisted_14)
      ])
    ]),
    _createElementVNode("div", _hoisted_15, [
      _createElementVNode("div", _hoisted_16, [
        _createElementVNode("div", _hoisted_17, [
          _createElementVNode("div", _hoisted_18, [
            (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.run.jobs, (job, index) => {
              return (_openBlock(), _createElementBlock("a", {
                class: _normalizeClass(["job-brief-item", parseInt(_ctx.jobIndex) === index ? 'selected' : '']),
                href: _ctx.run.link+'/jobs/'+index,
                key: job.id
              }, [
                _createElementVNode("div", _hoisted_20, [
                  _createVNode(_component_ActionRunStatus, {
                    "locale-status": _ctx.locale.status[job.status],
                    status: job.status
                  }, null, 8 /* PROPS */, ["locale-status", "status"]),
                  _createElementVNode("span", _hoisted_21, _toDisplayString(job.name), 1 /* TEXT */)
                ]),
                _createElementVNode("span", _hoisted_22, [
                  (job.canRerun)
                    ? (_openBlock(), _createBlock(_component_SvgIcon, {
                        key: 0,
                        name: "octicon-sync",
                        role: "button",
                        "data-tooltip-content": _ctx.locale.rerun,
                        class: "job-brief-rerun tw-mx-3 link-action",
                        "data-url": `${_ctx.run.link}/jobs/${index}/rerun`
                      }, null, 8 /* PROPS */, ["data-tooltip-content", "data-url"]))
                    : _createCommentVNode("v-if", true),
                  _createElementVNode("span", _hoisted_23, _toDisplayString(job.duration), 1 /* TEXT */)
                ])
              ], 10 /* CLASS, PROPS */, _hoisted_19))
            }), 128 /* KEYED_FRAGMENT */))
          ])
        ]),
        (_ctx.artifacts.length > 0)
          ? (_openBlock(), _createElementBlock("div", _hoisted_24, [
              _createElementVNode("div", _hoisted_25, _toDisplayString(_ctx.locale.artifactsTitle), 1 /* TEXT */),
              _createElementVNode("ul", _hoisted_26, [
                (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.artifacts, (artifact) => {
                  return (_openBlock(), _createElementBlock("li", {
                    class: "job-artifacts-item",
                    key: artifact.name
                  }, [
                    _createElementVNode("a", {
                      class: "job-artifacts-link",
                      target: "_blank",
                      href: _ctx.run.link+'/artifacts/'+artifact.name
                    }, [
                      _createVNode(_component_SvgIcon, {
                        name: "octicon-file",
                        class: "ui text black job-artifacts-icon"
                      }),
                      _createTextVNode(_toDisplayString(artifact.name), 1 /* TEXT */)
                    ], 8 /* PROPS */, _hoisted_27),
                    (_ctx.run.canDeleteArtifact)
                      ? (_openBlock(), _createElementBlock("a", {
                          key: 0,
                          onClick: $event => (_ctx.deleteArtifact(artifact.name)),
                          class: "job-artifacts-delete"
                        }, [
                          _createVNode(_component_SvgIcon, {
                            name: "octicon-trash",
                            class: "ui text black job-artifacts-icon"
                          })
                        ], 8 /* PROPS */, _hoisted_28))
                      : _createCommentVNode("v-if", true)
                  ]))
                }), 128 /* KEYED_FRAGMENT */))
              ])
            ]))
          : _createCommentVNode("v-if", true)
      ]),
      _createElementVNode("div", _hoisted_29, [
        _createElementVNode("div", _hoisted_30, [
          _createElementVNode("div", _hoisted_31, [
            _createElementVNode("h3", _hoisted_32, _toDisplayString(_ctx.currentJob.title), 1 /* TEXT */),
            _createElementVNode("p", _hoisted_33, _toDisplayString(_ctx.currentJob.detail), 1 /* TEXT */)
          ]),
          _createElementVNode("div", _hoisted_34, [
            _createElementVNode("div", {
              class: "ui top right pointing dropdown custom jump item",
              onClick: _cache[11] || (_cache[11] = _withModifiers($event => (_ctx.menuVisible = !_ctx.menuVisible), ["stop"]))
            }, [
              _createElementVNode("button", _hoisted_35, [
                _createVNode(_component_SvgIcon, {
                  name: "octicon-gear",
                  size: 18
                })
              ]),
              (_ctx.menuVisible)
                ? (_openBlock(), _createElementBlock("div", {
                    key: 0,
                    class: _normalizeClass(["menu transition action-job-menu", {visible: _ctx.menuVisible}])
                  }, [
                    _createElementVNode("a", {
                      class: "item",
                      tabindex: "0",
                      onClick: _cache[2] || (_cache[2] = $event => (_ctx.toggleTimeDisplay('seconds'))),
                      onKeyup: [
                        _cache[3] || (_cache[3] = _withKeys($event => (_ctx.toggleTimeDisplay('seconds')), ["space"])),
                        _cache[4] || (_cache[4] = _withKeys($event => (_ctx.toggleTimeDisplay('seconds')), ["enter"]))
                      ]
                    }, [
                      _createElementVNode("i", _hoisted_36, [
                        _createVNode(_component_SvgIcon, {
                          name: _ctx.timeVisible['log-time-seconds'] ? 'octicon-check' : 'gitea-empty-checkbox'
                        }, null, 8 /* PROPS */, ["name"])
                      ]),
                      _createTextVNode(" " + _toDisplayString(_ctx.locale.showLogSeconds), 1 /* TEXT */)
                    ], 32 /* NEED_HYDRATION */),
                    _createElementVNode("a", {
                      class: "item",
                      tabindex: "0",
                      onClick: _cache[5] || (_cache[5] = $event => (_ctx.toggleTimeDisplay('stamp'))),
                      onKeyup: [
                        _cache[6] || (_cache[6] = _withKeys($event => (_ctx.toggleTimeDisplay('stamp')), ["space"])),
                        _cache[7] || (_cache[7] = _withKeys($event => (_ctx.toggleTimeDisplay('stamp')), ["enter"]))
                      ]
                    }, [
                      _createElementVNode("i", _hoisted_37, [
                        _createVNode(_component_SvgIcon, {
                          name: _ctx.timeVisible['log-time-stamp'] ? 'octicon-check' : 'gitea-empty-checkbox'
                        }, null, 8 /* PROPS */, ["name"])
                      ]),
                      _createTextVNode(" " + _toDisplayString(_ctx.locale.showTimeStamps), 1 /* TEXT */)
                    ], 32 /* NEED_HYDRATION */),
                    _createElementVNode("a", {
                      class: "item",
                      tabindex: "0",
                      onClick: _cache[8] || (_cache[8] = $event => (_ctx.toggleFullScreen())),
                      onKeyup: [
                        _cache[9] || (_cache[9] = _withKeys($event => (_ctx.toggleFullScreen()), ["space"])),
                        _cache[10] || (_cache[10] = _withKeys($event => (_ctx.toggleFullScreen()), ["enter"]))
                      ]
                    }, [
                      _createElementVNode("i", _hoisted_38, [
                        _createVNode(_component_SvgIcon, {
                          name: _ctx.isFullScreen ? 'octicon-check' : 'gitea-empty-checkbox'
                        }, null, 8 /* PROPS */, ["name"])
                      ]),
                      _createTextVNode(" " + _toDisplayString(_ctx.locale.showFullScreen), 1 /* TEXT */)
                    ], 32 /* NEED_HYDRATION */),
                    _cache[12] || (_cache[12] = _createElementVNode("div", { class: "divider" }, null, -1 /* HOISTED */)),
                    _createElementVNode("a", {
                      class: _normalizeClass(['item', !_ctx.currentJob.steps.length ? 'disabled' : '']),
                      href: _ctx.run.link+'/jobs/'+_ctx.jobIndex+'/logs',
                      target: "_blank"
                    }, [
                      _createElementVNode("i", _hoisted_40, [
                        _createVNode(_component_SvgIcon, { name: "octicon-download" })
                      ]),
                      _createTextVNode(" " + _toDisplayString(_ctx.locale.downloadLogs), 1 /* TEXT */)
                    ], 10 /* CLASS, PROPS */, _hoisted_39)
                  ], 2 /* CLASS */))
                : _createCommentVNode("v-if", true)
            ])
          ])
        ]),
        (_ctx.currentJob.steps.length)
          ? (_openBlock(), _createElementBlock("div", _hoisted_41, [
              (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.currentJob.steps, (jobStep, i) => {
                return (_openBlock(), _createElementBlock("div", {
                  class: "job-step-section",
                  key: i
                }, [
                  _createElementVNode("div", {
                    class: _normalizeClass(["job-step-summary", [_ctx.currentJobStepsStates[i].expanded ? 'selected' : '', _ctx.isExpandable(jobStep.status) && 'step-expandable']]),
                    tabindex: "0",
                    onClick: _withModifiers($event => (_ctx.isExpandable(jobStep.status) && _ctx.toggleStepLogs(i)), ["stop"]),
                    onKeyup: [
                      _withKeys(_withModifiers($event => (_ctx.isExpandable(jobStep.status) && _ctx.toggleStepLogs(i)), ["stop"]), ["enter"]),
                      _withKeys(_withModifiers($event => (_ctx.isExpandable(jobStep.status) && _ctx.toggleStepLogs(i)), ["stop"]), ["space"])
                    ]
                  }, [
                    _createCommentVNode(" If the job is done and the job step log is loaded for the first time, show the loading icon\n                currentJobStepsStates[i].cursor === null means the log is loaded for the first time\n              "),
                    (_ctx.isDone(_ctx.run.status) && _ctx.currentJobStepsStates[i].expanded && _ctx.currentJobStepsStates[i].cursor === null)
                      ? (_openBlock(), _createBlock(_component_SvgIcon, {
                          key: 0,
                          name: "octicon-sync",
                          class: "tw-mr-2 job-status-rotate"
                        }))
                      : (_openBlock(), _createBlock(_component_SvgIcon, {
                          key: 1,
                          name: _ctx.currentJobStepsStates[i].expanded ? 'octicon-chevron-down': 'octicon-chevron-right',
                          class: _normalizeClass(['tw-mr-2', !_ctx.isExpandable(jobStep.status) && 'tw-invisible'])
                        }, null, 8 /* PROPS */, ["name", "class"])),
                    _createVNode(_component_ActionRunStatus, {
                      status: jobStep.status,
                      class: "tw-mr-2"
                    }, null, 8 /* PROPS */, ["status"]),
                    _createElementVNode("span", _hoisted_43, _toDisplayString(jobStep.summary), 1 /* TEXT */),
                    _createElementVNode("span", _hoisted_44, _toDisplayString(jobStep.duration), 1 /* TEXT */)
                  ], 42 /* CLASS, PROPS, NEED_HYDRATION */, _hoisted_42),
                  _createCommentVNode(" the log elements could be a lot, do not use v-if to destroy/reconstruct the DOM,\n            use native DOM elements for \"log line\" to improve performance, Vue is not suitable for managing so many reactive elements. "),
                  _withDirectives(_createElementVNode("div", {
                    class: "job-step-logs",
                    ref_for: true,
                    ref: "logs"
                  }, null, 512 /* NEED_PATCH */), [
                    [_vShow, _ctx.currentJobStepsStates[i].expanded]
                  ])
                ]))
              }), 128 /* KEYED_FRAGMENT */))
            ], 512 /* NEED_PATCH */))
          : _createCommentVNode("v-if", true)
      ])
    ])
  ]))
}