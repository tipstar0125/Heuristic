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

## 1手のみ差分更新`

- Node
    - track_id
    - refs
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
    - free(free[..at]:=nodesで現在使われているindex free[at..]:=使われていないindex)
    - at
    - cands

1. 初期nodeを作成(nodesの長さはビーム幅の2倍で初期化)
2. nodes(1つ前の世代)からcandを生成(score計算のみ、stateは生成しない)
3. 候補をビーム幅に制限するために、ソートして、削除(重複はスコアが高いものだけ残して削除)
4. 残ったcandをnodeにする
    - candの参照している親をカウント
    - nodesで参照カウントが0のものは親として使用されていないので、freeの使用されている領域を左に詰める
    - 親の参照カウンタをデクリメント
        - 参照カウンタが0の場合は、そのnodeをコピーせずに、差分更新(apply)
        - それ以外の場合は、コピーして、遷移(new_node)。nodesの使用領域をインクリメントして、末尾に新しいnodeを格納
    - scoreとhashは計算済みなので、stateのみを計算
    - parentのtrack_idとopをtrackに追加
    - trackの長さをnodeのtrack_idとする