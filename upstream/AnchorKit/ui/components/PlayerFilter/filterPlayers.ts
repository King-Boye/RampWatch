export interface PlayerFilters {
  region: string;
  position: string;
  level: string;
}

export interface Player {
  id: string;
  name: string;
  region: string;
  position: string;
  level: string;
  avatar?: string;
}

export const DEFAULT_FILTERS: PlayerFilters = {
  region: '',
  position: '',
  level: '',
};

export const FILTER_OPTIONS = {
  regions: [
    'North America',
    'Europe',
    'Asia Pacific',
    'Latin America',
    'Middle East & Africa',
  ],
  positions: ['Top', 'Jungle', 'Mid', 'Bot', 'Support'],
  levels: ['Bronze', 'Silver', 'Gold', 'Platinum', 'Diamond', 'Master'],
} as const;

const ALL_PLAYERS: Player[] = [
  { id: '1', name: 'Alice',  region: 'North America', position: 'Mid',     level: 'Diamond'  },
  { id: '2', name: 'Bob',    region: 'Europe',         position: 'Support', level: 'Gold'     },
  { id: '3', name: 'Carol',  region: 'Asia Pacific',   position: 'Top',     level: 'Master'   },
  { id: '4', name: 'Dave',   region: 'North America',  position: 'Jungle',  level: 'Platinum' },
  { id: '5', name: 'Eve',    region: 'Europe',         position: 'Bot',     level: 'Diamond'  },
  { id: '6', name: 'Frank',  region: 'Latin America',  position: 'Mid',     level: 'Gold'     },
  { id: '7', name: 'Grace',  region: 'Asia Pacific',   position: 'Support', level: 'Platinum' },
  { id: '8', name: 'Hank',   region: 'North America',  position: 'Top',     level: 'Silver'   },
];

export async function filterPlayers(filters: PlayerFilters): Promise<Player[]> {
  return ALL_PLAYERS.filter((player) => {
    if (filters.region    && player.region    !== filters.region)    return false;
    if (filters.position  && player.position  !== filters.position)  return false;
    if (filters.level     && player.level     !== filters.level)     return false;
    return true;
  });
}
