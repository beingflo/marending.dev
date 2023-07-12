export type Match = {
	gameId: string;
	players: Array<string>;
	// First entry in array is offensive player
	teamA: Array<string>;
	// First entry in array is offensive player
	teamB: Array<string>;
	startDate: Date;
	endDate: Date;
	teamAScore: number;
	teamBScore: number;
	goals?: Goals;
};

export type Goals = {
	scorer: Array<string>;
	time: Array<string>;
};
