export class Split {
	constructor(
		public a: Branch,
		public b: Branch,
		public vert: boolean,
		public ratio: number = 0.5
	) {}
}
export type Branch = View | Split;

export abstract class View {
	abstract duplicate(): View;
}
export class GradientView extends View {
	constructor(public top: string, public bottom: string) {
		super();
	}

	duplicate(): View {
		return new GradientView(this.top, this.bottom);
	}
}
export class RangeView extends View {
	constructor(public value: number) {
		super();
	}

	duplicate(): View {
		return new RangeView(this.value);
	}
}
