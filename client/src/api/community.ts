function range(start : number, stop : number, step : number): number[] {
    // https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/from#sequence_generator_range
    return Array.from({ length: (stop - start) / step + 1 }, (_, i) => start + i * step);
}

export function page_and_offset(rank : number): [number, number] {
    return [ Math.floor(rank / 10), rank % 10 ];
}

type RankedUser = { id: number, rank: number, name: string }

export class Community {
    communityId : number;

    constructor(communityId : number) {
        this.communityId = communityId;
    }

    async lastRank(): Promise<number> {
        return this.fetchSomeUsers('count') as Promise<number>;
    }

    async userRank(userId : number): Promise<number> {
        return this.fetchSomeUsers('rank&u=' + userId) as Promise<number>;
    }

    async fetchSomeUsers(searchParams: string): Promise<RankedUser[] | number> {
        let response = await fetch(`/api/communities/${this.communityId}/users?${searchParams}`);
        if(! response.ok) throw new Error(`Server error (${response.status})`);
        return response.json();
    }

    async fetchRankRange(start : number, stop : number): Promise<RankedUser[]> {
        let [[startPage, startOffset], [stopPage, stopOffset]] = [ page_and_offset(start), page_and_offset(stop) ];
        let pagesToFetch = range(startPage, stopPage, 1).map(i => this.fetchSomeUsers('page&n=' + i));
        let pages = await Promise.all(pagesToFetch) as RankedUser[][];
        pages[pages.length - 1].splice(stopOffset + 1);
        pages[0].splice(0, startOffset);
        let result = pages.reduce((li, elem) => li.concat(elem), []);
        result.forEach((page, i) => page.rank = start + i);
        return result;
    }

    async fetchMultiRankRange(ranges : [number, number][]): Promise<RankedUser[][]> {
        return Promise.all(ranges.map(range => this.fetchRankRange(...range))); // TODO: when the ranges overlap, do not fetch them multiple times
    }
}
