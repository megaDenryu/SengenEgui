//! 重ねる容器のまとまり。下地の上へ子を重ねる `重ね型` と、重ねる子の一覧・置き方・寄せ先・
//! ポインタが止まると隠す出し入れの規則を、重ねる容器だけが使う部品として1つのモジュールに収める。

mod children;
mod idle;
mod idle_group;
mod idle_memory;
mod idle_pointer;
mod overlay_container;
mod place;
mod position;

pub use idle::ポインタが止まると隠す指定;
pub use idle_group::ポインタが止まると隠す組;
pub use overlay_container::重ね型;
pub use position::重ねる位置;
