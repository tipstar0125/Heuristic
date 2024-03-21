# ビームサーチ

## ナイーブ

- Node
    - track_id
    - score
    - hash
    - state

- Cand
    - op(operation)
    - parent(nodesのindex)
    - score
    - hash

- BeamSearch
    - track(全世代のtrack_idとopを1次元で格納)
    - nodes
    - next_nodes

1. 初期nodeを作成(nodesに格納される)
2. nodes(1つ前の世代)からcandを生成(score計算のみ、stateは生成しない)
3. 候補をビーム幅に制限するために、ソートして、削除(重複はスコアが高いものだけ残して削除)
4. 残ったcandをnodeにする
    - next_nodesに格納して、最後にnodesとスワップ
    - scoreとhashは計算済みなので、stateのみを計算
    - parentのtrack_idとopをtrackに追加
    - trackの長さをnodeのtrack_idとする