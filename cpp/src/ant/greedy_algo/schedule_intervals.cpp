/*
区間スケジューリング問題:
特定の区間が決められた予定を実行できる最大値を求める.
制限: 同じ区間に実行できるものは一つのみ

上手くいかない例:
.選択肢から開始時間が一番早いものを選ぶことを繰り返す（反例）
.時間が最も短いものを選ぶことを繰り返す（反例）
.その仕事を選んだ時に、選べなくなる数が最も少ないものから選ぶことを繰り返す（反例）
*/