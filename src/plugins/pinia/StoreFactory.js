import {defineStore} from 'pinia';

/**
 * 创建一个 Pinia store 的工厂函数
 * @param {string} name - Store 的名称
 * @param {object} [initialState={}] - 初始状态
 * @param {boolean} [persistence=false] - 是否启用持久化
 * @returns {Function} - 返回一个 store 的定义函数
 */
export function createStore(name, initialState = {}, persistence = false) {
    if (!name || typeof name !== 'string') {
        throw new Error('名字有误');
    }

    return defineStore(name, {
        state: () => ({
            ...initialState,
        }),
        persist: persistence,
    });
}