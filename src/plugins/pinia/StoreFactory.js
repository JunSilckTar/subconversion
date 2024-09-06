import {defineStore} from 'pinia';

/**
 * 创建一个 Pinia store 的工厂函数
 * @param {string} name - Store 的名称
 * @param {object} [initialState={}] - 初始状态
 * @param {boolean} [persistence=false] - 是否启用持久化
 * @returns {Function} - 返回一个 store 的定义函数
 */

export function createStore(name, initialState, persistence) {
    if (name.length <= 0 || typeof name == 'undefined') throw new Error('名字有误');
    if (initialState == null || typeof initialState === 'undefined') initialState = {};
    if (persistence == null || typeof persistence === 'undefined') persistence = false;
    return defineStore(name, {
        state: () => ({
            ...initialState,
        }),
        persist: persistence,
    });
}