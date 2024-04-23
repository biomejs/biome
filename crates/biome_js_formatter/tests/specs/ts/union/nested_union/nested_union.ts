type Result =
    | {valid: false}
    | {
          valid: true;
          data:
              | {
                    otherValue: string;
                }
              | {
                    value: string;
                };
      };
