import { toDisplayString as _toDisplayString, createElementVNode as _createElementVNode, createTextVNode as _createTextVNode, normalizeClass as _normalizeClass, openBlock as _openBlock, createElementBlock as _createElementBlock, createCommentVNode as _createCommentVNode, vModelText as _vModelText, withDirectives as _withDirectives, resolveComponent as _resolveComponent, createVNode as _createVNode, mergeProps as _mergeProps, vShow as _vShow, renderList as _renderList, Fragment as _Fragment } from "vue"

const _hoisted_1 = {
  key: 0,
  class: "ui secondary stackable menu tabs-with-labels"
}
const _hoisted_2 = { class: "ui grey label tw-ml-2" }
const _hoisted_3 = { class: "ui grey label tw-ml-2" }
const _hoisted_4 = { class: "ui tab active list dashboard-repos" }
const _hoisted_5 = {
  key: 0,
  class: "ui top attached tw-mt-4 tw-flex tw-items-center"
}
const _hoisted_6 = { class: "tw-flex-1 tw-flex tw-items-center" }
const _hoisted_7 = { class: "ui grey label tw-ml-2" }
const _hoisted_8 = { class: "ui top attached segment repos-search" }
const _hoisted_9 = { class: "ui small fluid action left icon input" }
const _hoisted_10 = ["placeholder"]
const _hoisted_11 = ["title"]
const _hoisted_12 = { class: "menu" }
const _hoisted_13 = ["title"]
const _hoisted_14 = ["title"]
const _hoisted_15 = { class: "ui secondary pointing tabular borderless menu repos-filter" }
const _hoisted_16 = { class: "overflow-menu-items tw-justify-center" }
const _hoisted_17 = {
  key: 1,
  class: "ui attached table segment tw-rounded-b"
}
const _hoisted_18 = { class: "repo-owner-name-list" }
const _hoisted_19 = ["href"]
const _hoisted_20 = { class: "text truncate" }
const _hoisted_21 = { key: 0 }
const _hoisted_22 = ["href", "data-tooltip-content"]
const _hoisted_23 = {
  key: 0,
  class: "tw-text-center"
}
const _hoisted_24 = { class: "ui borderless pagination menu narrow tw-my-2" }
const _hoisted_25 = ["title"]
const _hoisted_26 = ["title"]
const _hoisted_27 = { class: "active item tw-py-1" }
const _hoisted_28 = ["title"]
const _hoisted_29 = ["title"]
const _hoisted_30 = {
  key: 1,
  class: "ui tab active list dashboard-orgs"
}
const _hoisted_31 = {
  key: 0,
  class: "ui attached table segment tw-rounded"
}
const _hoisted_32 = { class: "repo-owner-name-list" }
const _hoisted_33 = ["href"]
const _hoisted_34 = { class: "text truncate" }
const _hoisted_35 = {
  key: 0,
  class: "ui tiny basic label"
}
const _hoisted_36 = { class: "text light grey tw-flex tw-items-center tw-ml-2" }

export function render(_ctx, _cache, $props, $setup, $data, $options) {
  const _component_svg_icon = _resolveComponent("svg-icon")

  return (_openBlock(), _createElementBlock("div", null, [
    (!_ctx.isOrganization)
      ? (_openBlock(), _createElementBlock("div", _hoisted_1, [
          _createElementVNode("a", {
            class: _normalizeClass({item: true, active: _ctx.tab === 'repos'}),
            onClick: _cache[0] || (_cache[0] = $event => (_ctx.changeTab('repos')))
          }, [
            _createTextVNode(_toDisplayString(_ctx.textMyRepos) + " ", 1 /* TEXT */),
            _createElementVNode("span", _hoisted_2, _toDisplayString(_ctx.reposTotalCount), 1 /* TEXT */)
          ], 2 /* CLASS */),
          _createElementVNode("a", {
            class: _normalizeClass({item: true, active: _ctx.tab === 'organizations'}),
            onClick: _cache[1] || (_cache[1] = $event => (_ctx.changeTab('organizations')))
          }, [
            _createTextVNode(_toDisplayString(_ctx.textMyOrgs) + " ", 1 /* TEXT */),
            _createElementVNode("span", _hoisted_3, _toDisplayString(_ctx.organizationsTotalCount), 1 /* TEXT */)
          ], 2 /* CLASS */)
        ]))
      : _createCommentVNode("v-if", true),
    _withDirectives(_createElementVNode("div", _hoisted_4, [
      (_ctx.isOrganization)
        ? (_openBlock(), _createElementBlock("h4", _hoisted_5, [
            _createElementVNode("div", _hoisted_6, [
              _createTextVNode(_toDisplayString(_ctx.textMyRepos) + " ", 1 /* TEXT */),
              _createElementVNode("span", _hoisted_7, _toDisplayString(_ctx.reposTotalCount), 1 /* TEXT */)
            ])
          ]))
        : _createCommentVNode("v-if", true),
      _createElementVNode("div", _hoisted_8, [
        _createElementVNode("div", _hoisted_9, [
          _withDirectives(_createElementVNode("input", {
            type: "search",
            spellcheck: "false",
            maxlength: "255",
            onInput: _cache[2] || (_cache[2] = $event => (_ctx.changeReposFilter(_ctx.reposFilter))),
            "onUpdate:modelValue": _cache[3] || (_cache[3] = $event => ((_ctx.searchQuery) = $event)),
            ref: "search",
            onKeydown: _cache[4] || (_cache[4] = (...args) => (_ctx.reposFilterKeyControl && _ctx.reposFilterKeyControl(...args))),
            placeholder: _ctx.textSearchRepos
          }, null, 40 /* PROPS, NEED_HYDRATION */, _hoisted_10), [
            [_vModelText, _ctx.searchQuery]
          ]),
          _createElementVNode("i", {
            class: _normalizeClass(["icon loading-icon-3px", {'is-loading': _ctx.isLoading}])
          }, [
            _createVNode(_component_svg_icon, {
              name: "octicon-search",
              size: 16
            })
          ], 2 /* CLASS */),
          _createElementVNode("div", {
            class: "ui dropdown icon button",
            title: _ctx.textFilter
          }, [
            _createVNode(_component_svg_icon, {
              name: "octicon-filter",
              size: 16
            }),
            _createElementVNode("div", _hoisted_12, [
              _createElementVNode("a", {
                class: "item",
                onClick: _cache[5] || (_cache[5] = $event => (_ctx.toggleArchivedFilter()))
              }, [
                _createElementVNode("div", {
                  class: "ui checkbox",
                  ref: "checkboxArchivedFilter",
                  title: _ctx.checkboxArchivedFilterTitle
                }, [
                  _createCommentVNode("the \"tw-pointer-events-none\" is necessary to prevent the checkbox from handling user's input,\n                      otherwise if the \"input\" handles click event for intermediate status, it breaks the internal state"),
                  _createElementVNode("input", _mergeProps({
                    type: "checkbox",
                    class: "tw-pointer-events-none"
                  }, _ctx.checkboxArchivedFilterProps), null, 16 /* FULL_PROPS */),
                  _createElementVNode("label", null, [
                    _createVNode(_component_svg_icon, {
                      name: "octicon-archive",
                      size: 16,
                      class: "tw-mr-1"
                    }),
                    _createTextVNode(" " + _toDisplayString(_ctx.textShowArchived), 1 /* TEXT */)
                  ])
                ], 8 /* PROPS */, _hoisted_13)
              ]),
              _createElementVNode("a", {
                class: "item",
                onClick: _cache[6] || (_cache[6] = $event => (_ctx.togglePrivateFilter()))
              }, [
                _createElementVNode("div", {
                  class: "ui checkbox",
                  ref: "checkboxPrivateFilter",
                  title: _ctx.checkboxPrivateFilterTitle
                }, [
                  _createElementVNode("input", _mergeProps({
                    type: "checkbox",
                    class: "tw-pointer-events-none"
                  }, _ctx.checkboxPrivateFilterProps), null, 16 /* FULL_PROPS */),
                  _createElementVNode("label", null, [
                    _createVNode(_component_svg_icon, {
                      name: "octicon-lock",
                      size: 16,
                      class: "tw-mr-1"
                    }),
                    _createTextVNode(" " + _toDisplayString(_ctx.textShowPrivate), 1 /* TEXT */)
                  ])
                ], 8 /* PROPS */, _hoisted_14)
              ])
            ])
          ], 8 /* PROPS */, _hoisted_11)
        ]),
        _createElementVNode("overflow-menu", _hoisted_15, [
          _createElementVNode("div", _hoisted_16, [
            _createElementVNode("a", {
              class: _normalizeClass(["item", {active: _ctx.reposFilter === 'all'}]),
              tabindex: "0",
              onClick: _cache[7] || (_cache[7] = $event => (_ctx.changeReposFilter('all')))
            }, [
              _createTextVNode(_toDisplayString(_ctx.textAll) + " ", 1 /* TEXT */),
              _withDirectives(_createElementVNode("div", { class: "ui circular mini grey label" }, _toDisplayString(_ctx.repoTypeCount), 513 /* TEXT, NEED_PATCH */), [
                [_vShow, _ctx.reposFilter === 'all']
              ])
            ], 2 /* CLASS */),
            _createElementVNode("a", {
              class: _normalizeClass(["item", {active: _ctx.reposFilter === 'sources'}]),
              tabindex: "0",
              onClick: _cache[8] || (_cache[8] = $event => (_ctx.changeReposFilter('sources')))
            }, [
              _createTextVNode(_toDisplayString(_ctx.textSources) + " ", 1 /* TEXT */),
              _withDirectives(_createElementVNode("div", { class: "ui circular mini grey label" }, _toDisplayString(_ctx.repoTypeCount), 513 /* TEXT, NEED_PATCH */), [
                [_vShow, _ctx.reposFilter === 'sources']
              ])
            ], 2 /* CLASS */),
            _createElementVNode("a", {
              class: _normalizeClass(["item", {active: _ctx.reposFilter === 'forks'}]),
              tabindex: "0",
              onClick: _cache[9] || (_cache[9] = $event => (_ctx.changeReposFilter('forks')))
            }, [
              _createTextVNode(_toDisplayString(_ctx.textForks) + " ", 1 /* TEXT */),
              _withDirectives(_createElementVNode("div", { class: "ui circular mini grey label" }, _toDisplayString(_ctx.repoTypeCount), 513 /* TEXT, NEED_PATCH */), [
                [_vShow, _ctx.reposFilter === 'forks']
              ])
            ], 2 /* CLASS */),
            (_ctx.isMirrorsEnabled)
              ? (_openBlock(), _createElementBlock("a", {
                  key: 0,
                  class: _normalizeClass(["item", {active: _ctx.reposFilter === 'mirrors'}]),
                  tabindex: "0",
                  onClick: _cache[10] || (_cache[10] = $event => (_ctx.changeReposFilter('mirrors')))
                }, [
                  _createTextVNode(_toDisplayString(_ctx.textMirrors) + " ", 1 /* TEXT */),
                  _withDirectives(_createElementVNode("div", { class: "ui circular mini grey label" }, _toDisplayString(_ctx.repoTypeCount), 513 /* TEXT, NEED_PATCH */), [
                    [_vShow, _ctx.reposFilter === 'mirrors']
                  ])
                ], 2 /* CLASS */))
              : _createCommentVNode("v-if", true),
            _createElementVNode("a", {
              class: _normalizeClass(["item", {active: _ctx.reposFilter === 'collaborative'}]),
              tabindex: "0",
              onClick: _cache[11] || (_cache[11] = $event => (_ctx.changeReposFilter('collaborative')))
            }, [
              _createTextVNode(_toDisplayString(_ctx.textCollaborative) + " ", 1 /* TEXT */),
              _withDirectives(_createElementVNode("div", { class: "ui circular mini grey label" }, _toDisplayString(_ctx.repoTypeCount), 513 /* TEXT, NEED_PATCH */), [
                [_vShow, _ctx.reposFilter === 'collaborative']
              ])
            ], 2 /* CLASS */)
          ])
        ])
      ]),
      (_ctx.repos.length)
        ? (_openBlock(), _createElementBlock("div", _hoisted_17, [
            _createElementVNode("ul", _hoisted_18, [
              (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.repos, (repo, index) => {
                return (_openBlock(), _createElementBlock("li", {
                  class: _normalizeClass(["tw-flex tw-items-center tw-py-2", {'active': index === _ctx.activeIndex}]),
                  key: repo.id
                }, [
                  _createElementVNode("a", {
                    class: "repo-list-link muted",
                    href: repo.link
                  }, [
                    _createVNode(_component_svg_icon, {
                      name: _ctx.repoIcon(repo),
                      size: 16,
                      class: "repo-list-icon"
                    }, null, 8 /* PROPS */, ["name"]),
                    _createElementVNode("div", _hoisted_20, _toDisplayString(repo.full_name), 1 /* TEXT */),
                    (repo.archived)
                      ? (_openBlock(), _createElementBlock("div", _hoisted_21, [
                          _createVNode(_component_svg_icon, {
                            name: "octicon-archive",
                            size: 16
                          })
                        ]))
                      : _createCommentVNode("v-if", true)
                  ], 8 /* PROPS */, _hoisted_19),
                  (repo.latest_commit_status)
                    ? (_openBlock(), _createElementBlock("a", {
                        key: 0,
                        class: "tw-flex tw-items-center",
                        href: repo.latest_commit_status.TargetURL,
                        "data-tooltip-content": repo.locale_latest_commit_status
                      }, [
                        _createCommentVNode(" the commit status icon logic is taken from templates/repo/commit_status.tmpl "),
                        _createVNode(_component_svg_icon, {
                          name: _ctx.statusIcon(repo.latest_commit_status.State),
                          class: _normalizeClass('tw-ml-2 commit-status icon text ' + _ctx.statusColor(repo.latest_commit_status.State)),
                          size: 16
                        }, null, 8 /* PROPS */, ["name", "class"])
                      ], 8 /* PROPS */, _hoisted_22))
                    : _createCommentVNode("v-if", true)
                ], 2 /* CLASS */))
              }), 128 /* KEYED_FRAGMENT */))
            ]),
            (_ctx.showMoreReposLink)
              ? (_openBlock(), _createElementBlock("div", _hoisted_23, [
                  _cache[16] || (_cache[16] = _createElementVNode("div", { class: "divider tw-my-0" }, null, -1 /* HOISTED */)),
                  _createElementVNode("div", _hoisted_24, [
                    _createElementVNode("a", {
                      class: _normalizeClass(["item navigation tw-py-1", {'disabled': _ctx.page === 1}]),
                      onClick: _cache[12] || (_cache[12] = $event => (_ctx.changePage(1))),
                      title: _ctx.textFirstPage
                    }, [
                      _createVNode(_component_svg_icon, {
                        name: "gitea-double-chevron-left",
                        size: 16,
                        class: "tw-mr-1"
                      })
                    ], 10 /* CLASS, PROPS */, _hoisted_25),
                    _createElementVNode("a", {
                      class: _normalizeClass(["item navigation tw-py-1", {'disabled': _ctx.page === 1}]),
                      onClick: _cache[13] || (_cache[13] = $event => (_ctx.changePage(_ctx.page - 1))),
                      title: _ctx.textPreviousPage
                    }, [
                      _createVNode(_component_svg_icon, {
                        name: "octicon-chevron-left",
                        size: 16,
                        class: "tw-mr-1"
                      })
                    ], 10 /* CLASS, PROPS */, _hoisted_26),
                    _createElementVNode("a", _hoisted_27, _toDisplayString(_ctx.page), 1 /* TEXT */),
                    _createElementVNode("a", {
                      class: _normalizeClass(["item navigation", {'disabled': _ctx.page === _ctx.finalPage}]),
                      onClick: _cache[14] || (_cache[14] = $event => (_ctx.changePage(_ctx.page + 1))),
                      title: _ctx.textNextPage
                    }, [
                      _createVNode(_component_svg_icon, {
                        name: "octicon-chevron-right",
                        size: 16,
                        class: "tw-ml-1"
                      })
                    ], 10 /* CLASS, PROPS */, _hoisted_28),
                    _createElementVNode("a", {
                      class: _normalizeClass(["item navigation tw-py-1", {'disabled': _ctx.page === _ctx.finalPage}]),
                      onClick: _cache[15] || (_cache[15] = $event => (_ctx.changePage(_ctx.finalPage))),
                      title: _ctx.textLastPage
                    }, [
                      _createVNode(_component_svg_icon, {
                        name: "gitea-double-chevron-right",
                        size: 16,
                        class: "tw-ml-1"
                      })
                    ], 10 /* CLASS, PROPS */, _hoisted_29)
                  ])
                ]))
              : _createCommentVNode("v-if", true)
          ]))
        : _createCommentVNode("v-if", true)
    ], 512 /* NEED_PATCH */), [
      [_vShow, _ctx.tab === 'repos']
    ]),
    (!_ctx.isOrganization)
      ? _withDirectives((_openBlock(), _createElementBlock("div", _hoisted_30, [
          (_ctx.organizations.length)
            ? (_openBlock(), _createElementBlock("div", _hoisted_31, [
                _createElementVNode("ul", _hoisted_32, [
                  (_openBlock(true), _createElementBlock(_Fragment, null, _renderList(_ctx.organizations, (org) => {
                    return (_openBlock(), _createElementBlock("li", {
                      class: "tw-flex tw-items-center tw-py-2",
                      key: org.name
                    }, [
                      _createElementVNode("a", {
                        class: "repo-list-link muted",
                        href: _ctx.subUrl + '/' + encodeURIComponent(org.name)
                      }, [
                        _createVNode(_component_svg_icon, {
                          name: "octicon-organization",
                          size: 16,
                          class: "repo-list-icon"
                        }),
                        _createElementVNode("div", _hoisted_34, _toDisplayString(org.name), 1 /* TEXT */),
                        _createElementVNode("div", null, [
                          _createCommentVNode(" div to prevent underline of label on hover "),
                          (org.org_visibility !== 'public')
                            ? (_openBlock(), _createElementBlock("span", _hoisted_35, _toDisplayString(org.org_visibility === 'limited' ? _ctx.textOrgVisibilityLimited: _ctx.textOrgVisibilityPrivate), 1 /* TEXT */))
                            : _createCommentVNode("v-if", true)
                        ])
                      ], 8 /* PROPS */, _hoisted_33),
                      _createElementVNode("div", _hoisted_36, [
                        _createTextVNode(_toDisplayString(org.num_repos) + " ", 1 /* TEXT */),
                        _createVNode(_component_svg_icon, {
                          name: "octicon-repo",
                          size: 16,
                          class: "tw-ml-1 tw-mt-0.5"
                        })
                      ])
                    ]))
                  }), 128 /* KEYED_FRAGMENT */))
                ])
              ]))
            : _createCommentVNode("v-if", true)
        ], 512 /* NEED_PATCH */)), [
          [_vShow, _ctx.tab === 'organizations']
        ])
      : _createCommentVNode("v-if", true)
  ]))
}