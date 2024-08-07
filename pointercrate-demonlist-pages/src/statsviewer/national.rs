use crate::statsviewer::{stats_viewer_html, StatsViewerRow};
use maud::{html, Markup};
use pointercrate_core_pages::{head::HeadLike, PageFragment};

pub fn nation_based_stats_viewer() -> PageFragment {
    use pointercrate_core_pages::{versioned_import, with_version_string};

    PageFragment::new(
        "Nation Stats Viewer",
        "The pointercrate nation stats viewer, ranking how well each nation's players are doing in their quest to collectively complete \
         the entire demonlist!",
    )
    .module(with_version_string!("/static/demonlist/js/modules/statsviewer.js"))
    .module(with_version_string!("/static/demonlist/js/statsviewer/nation.js"))
    .import(versioned_import!("/static/core/js/modules/form.js"))
    .import(versioned_import!("/static/demonlist/js/modules/demonlist.js"))
    .import(versioned_import!("/static/demonlist/js/modules/statsviewer.js"))
    .stylesheet(with_version_string!("/static/demonlist/css/statsviewer.css"))
    .stylesheet(with_version_string!("/static/core/css/sidebar.css"))
    .body(nation_based_stats_viewer_html())
}

fn nation_based_stats_viewer_html() -> Markup {
    let mut rows = super::standard_stats_viewer_rows();

    rows[0].0.insert(1, ("Players", "players"));
    rows.push(StatsViewerRow(vec![("Unbeaten demons", "unbeaten")]));

    html! {
        nav.flex.wrap.m-center.fade #statsviewers style="text-align: center;" {
            a.button.dark-gray.hover.no-shadow href="/demonlist/statsviewer/"{
                b {"Individual"}
            }
            a.button.dark-gray.hover.no-shadow href="/demonlist/statsviewer/nations/" {
                b {"Nations"}
            }
        }
        div.flex.m-center.container {
            main.left {
                (stats_viewer_html(None, rows))
            }
            aside.right {
                (super::continent_panel())
            }
        }
    }
}
