type BagOfPromises = {
    [property: string]: Promise<void>;
};

let bag: BagOfPromises = {};
bag.canYouFindMe;

const { anotherOne } = bag;
anotherOne;
