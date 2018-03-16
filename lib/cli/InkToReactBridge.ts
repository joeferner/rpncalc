import {h} from "ink";

export default class React {
    static createElement() {
        return h.apply(null, arguments);
    }
}
