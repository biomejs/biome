class T {
    // This aggregates all values per `timestamp`
    calculateTotalsPerTimestamp(
      getName: (
        timestamp: number,
        countArray: {count: number}[],
        i: number
      ) => number = timestamp => timestamp * 1000
    ): SeriesDataUnit[] {
    }

    calculateTotalsPerTimestamp(
      getName: (
        timestamp: number
      ) => number = 
      timestamp => timestamp * 1000
    ) {


    }
  }