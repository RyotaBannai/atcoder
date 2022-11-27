/**
 * 計算幾何学
 *
 * 単一ファイルで使うときは、以下の変更を行う
 * ・全　geo_lib を消す e.g. use crate::geo_lib::... -> use crate::...
 *
 *
 * AOJ 提出時
 * ・このライブラリを入れて単一ファイルにする（AtCoder で使っているライブラリは使わないため取り除く（まずファイルサイズが大きくて通らない））
 * e.g.
 * cargo equip --bin [bin_name] --remove docs --minify libs --exclude-atcoder-crates --exclude easy-ext ac-library-rs | pbcopy
 *
 */
pub mod circle;
pub mod linear_equation;
pub mod manhattan_geometry;
pub mod polygon;
pub mod vector;
