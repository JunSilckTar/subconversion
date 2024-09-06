import {storeToRefs} from 'pinia';
import {useMainStore} from "./Store.js";

/**
 * 封装增删改查和 flush 刷新方法
 * @param store Pinia 的 Store
 */
export function usePinia(store) {

    if (store == null || typeof store === 'undefined') store = useMainStore()

    function add(key, value) {
        if (store.$state) {
            store.$patch((state) => {
                state[key] = value;
            });
        } else {
            console.error('add wrong');
        }
        return this;  // 支持链式调用
    }

    function remove(key) {
        if (store.$state) {
            store.$patch((state) => {
                delete state[key];
            });
        } else {
            console.error('remove wrong');
        }
        return this;
    }

    function update(key, value) {
        if (store.$state) {
            store.$patch((state) => {
                state[key] = value;
            });
        } else {
            console.error('update wrong');
        }
        return this;
    }

    function query(key) {
        if (store.$state) {
            const {[key]: value} = storeToRefs(store);
            return value.value;  // 获取实际的值
        }
        console.error('query wrong');
        return null;
    }

    function flush() {
        if (store && store.$persist) {
            store.$persist();
        } else {
            console.error('flush function is not available');
        }
        return this;  // 支持链式调用
    }

    return {
        add,
        remove,
        update,
        query,
        flush,
    };
}

