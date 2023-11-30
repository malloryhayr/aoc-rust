export interface Solution {
	value: string;
	time: number;
}

export type Year = string[];
export interface Input {
	[key: string]: Year;
}
