/**
 * クエリー
 *
 * 単一ファイルで使うときは、以下の変更を行う
 * ・全 query_lib を消す e.g. use crate::query_lib::... -> use crate::...
 *
 *
 * AOJ 提出時
 * ・このライブラリを入れて単一ファイルにする（AtCoder で使っているライブラリは使わないため取り除く（まずファイルサイズが大きくて通らない））
 * e.g. cargo equip --bin [bin_name] --remove docs --minify libs --exclude-atcoder-crates --exclude easy-ext ac-library-rs | pbcopy
 *
 */
pub mod seg_tree;
