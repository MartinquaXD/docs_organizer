import {History} from 'history';

export interface ThunkExtraArgs {
    history: History
}

export function mergePropsFunc<A extends object, B extends object, C extends object>(a: A, b: B, c: C): A & B & C {
    return Object.assign({}, a, b, c)
}