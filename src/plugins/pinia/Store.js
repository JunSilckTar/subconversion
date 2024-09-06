
import {createStore} from "./StoreFactory.js";


const Store = Object.freeze({
    MAIN: 'mainStore',
    COMPONENT: 'componentStore',
    DATA: 'dataStore',
});

// 默认的mainStore
export const useMainStore = createStore(Store.MAIN, {
    tempLog: '' // 临时日志数据
}, false);

// 创建 dataStore，存储重要数据，启用持久化
export const useDataStore = createStore(Store.DATA, {
    importantData: '',  // 重要的程序数据
    logs: []            // 存储日志
}, true);

export const useComponentStore = createStore(Store.COMPONENT, {

},false);
