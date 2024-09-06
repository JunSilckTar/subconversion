import {useMainStore} from './Store.js';

/**
 * 封装增删改查和 flush 刷新方法
 * @param store Pinia 的 Store
 */
export function usePinia(store = useMainStore()) {

    function add(key, value) {
        if (store && store.$state) {
            store.$patch((state) => {
                state[key] = value;
            });
        } else {
            console.error('add failed');
        }
        return this;  // 支持链式调用
    }

    function remove(key) {
        if (store && store.$state) {
            store.$patch((state) => {
                delete state[key];
            });
        } else {
            console.error('remove failed');
        }
        return this;
    }

    function update(key, value) {
        if (store && store.$state) {
            store.$patch((state) => {
                state[key] = value;
            });
        } else {
            console.error('update failed');
        }
        return this;
    }

    function query(key) {
        if (store && store.$state) {
            return store.$state[key]; // 直接从 state 获取值
        }
        console.error('query failed');
        return null;
    }

    function flush() {
        if (store && typeof store.$persist === 'function') {
            store.$persist(); // 确保使用持久化插件
        } else {
            console.error('flush failed or persist not available');
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