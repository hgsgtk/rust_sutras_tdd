# はじめに
著書「テスト駆動開発」の第1章〜第17章の写経をrustで書いたものです。

- [書籍「テスト駆動開発」をRustで書く #1 （chapter1...3）](https://qiita.com/Khigashiguchi/items/dfc382d9d2f988522721)
- [書籍「テスト駆動開発」をRustで書く #2 （chapter4...11）](https://qiita.com/Khigashiguchi/items/6041308a46b30e849648)
- [書籍「テスト駆動開発」をRustで書く #3 （chapter7...11）](https://qiita.com/Khigashiguchi/items/61ffa5da34d957df01f1)

# 著書内容
## Chapters

- Chapter/1 仮実装
- Chapter/2 明白な実装
- Chapter/3 三角推量
- Chapter/4 意図を語るテスト
- Chapter/5 原則をあえて破るとき
- Chapter/6 テスト不足に気づいたら
- Chapter/7 疑念をテストに翻訳する
- Chapter/8 実装を隠す
- Chapter/9 歩幅の調整
- Chapter/10 テストに聞いてみる
- Chapter/11 不要になったら消す

### TODO LIST in Chapters
著書内でTDD進めていく際のTODOリストです。

#### TODO After chapter/12
- [ ] $5 + 10CHF = $10
- [ ] $5 + $5 = $10

#### TODO Before chapter/11
- [ ] $5 + 10CHF = $10
- [x] $5 * 2 = $10
- [x] amountをprivateにする（すでにprivateだった）
- [x] Dollarの副作用どうする
- [ ] Moneyの丸め処理どうする
- [x] equals()の実装
- [ ] hashCode()の実装
- [ ] nullとの等価性比較
- [ ] 他のオブジェクトとの等価性比較
- [x] 5CHF * 2 = 10CHF
- [x] DollarとFrancの重複
- [x] equalsの一般化
- [x] timesの一般化
- [x] FrancとDollarを比較する
- [x] 通貨の概念
- [x] testFrancMultiplicationを削除する？
