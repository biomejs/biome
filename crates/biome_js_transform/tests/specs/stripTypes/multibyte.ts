type 型 = string; // 残るコメント
const 値: 型 = 'あ';

interface インターフェース {
	// 消えるコメント
	フィールド: 型;
}

const 配列: Array<
	// これも消える
	型
> = ['い'];
