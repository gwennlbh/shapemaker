let wasm;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}
/**
 * @param {Color} c
 * @returns {string}
 */
export function color_name(c) {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.color_name(c);
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_2.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * @param {number} opacity
 * @param {Color} color
 */
export function render_image(opacity, color) {
    const ret = wasm.render_image(opacity, color);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

export function map_to_midi_controller() {
    wasm.map_to_midi_controller();
}

/**
 * @param {string} selector
 */
export function render_canvas_into(selector) {
    const ptr0 = passStringToWasm0(selector, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.render_canvas_into(ptr0, len0);
}

/**
 * @param {string} selector
 */
export function render_canvas_at(selector) {
    const ptr0 = passStringToWasm0(selector, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.render_canvas_at(ptr0, len0);
}

/**
 * @returns {string}
 */
export function render_canvas() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.render_canvas();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}
/**
 * @param {ColorMapping} palette
 */
export function set_palette(palette) {
    _assertClass(palette, ColorMapping);
    var ptr0 = palette.__destroy_into_raw();
    wasm.set_palette(ptr0);
}

/**
 * @param {string} name
 * @returns {LayerWeb}
 */
export function get_layer(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_layer(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return LayerWeb.__wrap(ret[0]);
}

/**
 * @param {string} name
 * @returns {LayerWeb}
 */
export function random_linelikes(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.random_linelikes(ptr0, len0);
    return LayerWeb.__wrap(ret);
}

/**
 * @param {string} name
 * @returns {LayerWeb}
 */
export function new_layer(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.new_layer(ptr0, len0);
    return LayerWeb.__wrap(ret);
}

let cachedFloat32ArrayMemory0 = null;

function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
 * @param {Color | null} [except]
 * @returns {Color}
 */
export function random_color(except) {
    const ret = wasm.random_color(isLikeNone(except) ? 12 : except);
    return ret;
}

/**
 * @param {string} s
 * @returns {string}
 */
export function slugify(s) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(s, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.slugify(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11}
 */
export const Color = Object.freeze({
    Black: 0, "0": "Black",
    White: 1, "1": "White",
    Red: 2, "2": "Red",
    Green: 3, "3": "Green",
    Blue: 4, "4": "Blue",
    Yellow: 5, "5": "Yellow",
    Orange: 6, "6": "Orange",
    Purple: 7, "7": "Purple",
    Brown: 8, "8": "Brown",
    Cyan: 9, "9": "Cyan",
    Pink: 10, "10": "Pink",
    Gray: 11, "11": "Gray",
});
/**
 * @enum {0 | 1 | 2}
 */
export const FilterType = Object.freeze({
    Glow: 0, "0": "Glow",
    NaturalShadow: 1, "1": "NaturalShadow",
    Saturation: 2, "2": "Saturation",
});
/**
 * @enum {0 | 1}
 */
export const MidiEvent = Object.freeze({
    Note: 0, "0": "Note",
    ControlChange: 1, "1": "ControlChange",
});
/**
 * @enum {0 | 1 | 2 | 3}
 */
export const TransformationType = Object.freeze({
    Scale: 0, "0": "Scale",
    Rotate: 1, "1": "Rotate",
    Skew: 2, "2": "Skew",
    Matrix: 3, "3": "Matrix",
});

const ColorMappingFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_colormapping_free(ptr >>> 0, 1));

export class ColorMapping {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ColorMapping.prototype);
        obj.__wbg_ptr = ptr;
        ColorMappingFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ColorMappingFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_colormapping_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get black() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_black(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set black(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_black(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get white() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_white(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set white(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_white(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get red() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_red(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set red(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_red(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get green() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_green(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set green(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_green(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get blue() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_blue(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set blue(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_blue(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get yellow() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_yellow(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set yellow(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_yellow(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get orange() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_orange(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set orange(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_orange(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get purple() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_purple(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set purple(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_purple(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get brown() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_brown(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set brown(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_brown(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get cyan() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_cyan(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set cyan(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_cyan(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get pink() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_pink(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set pink(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_pink(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get gray() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_colormapping_gray(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set gray(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_colormapping_gray(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {ColorMapping}
     */
    static default() {
        const ret = wasm.colormapping_default();
        return ColorMapping.__wrap(ret);
    }
    /**
     * @param {string} content
     * @returns {ColorMapping}
     */
    static from_json(content) {
        const ptr0 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.colormapping_from_json(ptr0, len0);
        return ColorMapping.__wrap(ret);
    }
    /**
     * @param {string} content
     * @returns {ColorMapping}
     */
    static from_css(content) {
        const ptr0 = passStringToWasm0(content, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.colormapping_from_css(ptr0, len0);
        return ColorMapping.__wrap(ret);
    }
}

const FilterFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_filter_free(ptr >>> 0, 1));

export class Filter {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Filter.prototype);
        obj.__wbg_ptr = ptr;
        FilterFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FilterFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_filter_free(ptr, 0);
    }
    /**
     * @returns {FilterType}
     */
    get kind() {
        const ret = wasm.__wbg_get_filter_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {FilterType} arg0
     */
    set kind(arg0) {
        wasm.__wbg_set_filter_kind(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get parameter() {
        const ret = wasm.__wbg_get_filter_parameter(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set parameter(arg0) {
        wasm.__wbg_set_filter_parameter(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {string}
     */
    name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.filter_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} intensity
     * @returns {Filter}
     */
    static glow(intensity) {
        const ret = wasm.filter_glow(intensity);
        return Filter.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.filter_id(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const LayerWebFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_layerweb_free(ptr >>> 0, 1));

export class LayerWeb {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(LayerWeb.prototype);
        obj.__wbg_ptr = ptr;
        LayerWebFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LayerWebFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_layerweb_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_layerweb_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_layerweb_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    render() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.layerweb_render(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} selector
     */
    render_into(selector) {
        const ptr0 = passStringToWasm0(selector, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.layerweb_render_into(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} selector
     */
    render_at(selector) {
        const ptr = this.__destroy_into_raw();
        const ptr0 = passStringToWasm0(selector, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.layerweb_render_at(ptr, ptr0, len0);
    }
    /**
     * @param {Color} color
     * @param {number | null | undefined} opacity
     * @param {Filter} filter
     */
    paint_all(color, opacity, filter) {
        _assertClass(filter, Filter);
        var ptr0 = filter.__destroy_into_raw();
        wasm.layerweb_paint_all(this.__wbg_ptr, color, isLikeNone(opacity) ? 0x100000001 : Math.fround(opacity), ptr0);
    }
    /**
     * @param {string} name
     * @returns {LayerWeb}
     */
    static random(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.layerweb_random(ptr0, len0);
        return LayerWeb.__wrap(ret);
    }
    /**
     * @param {string} name
     * @param {Point} start
     * @param {Point} end
     * @param {number} thickness
     * @param {Color} color
     */
    new_line(name, start, end, thickness, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(start, Point);
        var ptr1 = start.__destroy_into_raw();
        _assertClass(end, Point);
        var ptr2 = end.__destroy_into_raw();
        wasm.layerweb_new_line(this.__wbg_ptr, ptr0, len0, ptr1, ptr2, thickness, color);
    }
    /**
     * @param {string} name
     * @param {Point} start
     * @param {Point} end
     * @param {number} thickness
     * @param {Color} color
     */
    new_curve_outward(name, start, end, thickness, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(start, Point);
        var ptr1 = start.__destroy_into_raw();
        _assertClass(end, Point);
        var ptr2 = end.__destroy_into_raw();
        wasm.layerweb_new_curve_outward(this.__wbg_ptr, ptr0, len0, ptr1, ptr2, thickness, color);
    }
    /**
     * @param {string} name
     * @param {Point} start
     * @param {Point} end
     * @param {number} thickness
     * @param {Color} color
     */
    new_curve_inward(name, start, end, thickness, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(start, Point);
        var ptr1 = start.__destroy_into_raw();
        _assertClass(end, Point);
        var ptr2 = end.__destroy_into_raw();
        wasm.layerweb_new_curve_inward(this.__wbg_ptr, ptr0, len0, ptr1, ptr2, thickness, color);
    }
    /**
     * @param {string} name
     * @param {Point} center
     * @param {Color} color
     */
    new_small_circle(name, center, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(center, Point);
        var ptr1 = center.__destroy_into_raw();
        wasm.layerweb_new_small_circle(this.__wbg_ptr, ptr0, len0, ptr1, color);
    }
    /**
     * @param {string} name
     * @param {Point} center
     * @param {Color} color
     */
    new_dot(name, center, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(center, Point);
        var ptr1 = center.__destroy_into_raw();
        wasm.layerweb_new_dot(this.__wbg_ptr, ptr0, len0, ptr1, color);
    }
    /**
     * @param {string} name
     * @param {Point} center
     * @param {Color} color
     */
    new_big_circle(name, center, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(center, Point);
        var ptr1 = center.__destroy_into_raw();
        wasm.layerweb_new_big_circle(this.__wbg_ptr, ptr0, len0, ptr1, color);
    }
    /**
     * @param {string} name
     * @param {Point} anchor
     * @param {string} text
     * @param {number} font_size
     * @param {Color} color
     */
    new_text(name, anchor, text, font_size, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(anchor, Point);
        var ptr1 = anchor.__destroy_into_raw();
        const ptr2 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len2 = WASM_VECTOR_LEN;
        wasm.layerweb_new_text(this.__wbg_ptr, ptr0, len0, ptr1, ptr2, len2, font_size, color);
    }
    /**
     * @param {string} name
     * @param {Point} topleft
     * @param {Point} bottomright
     * @param {Color} color
     */
    new_rectangle(name, topleft, bottomright, color) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(topleft, Point);
        var ptr1 = topleft.__destroy_into_raw();
        _assertClass(bottomright, Point);
        var ptr2 = bottomright.__destroy_into_raw();
        wasm.layerweb_new_rectangle(this.__wbg_ptr, ptr0, len0, ptr1, ptr2, color);
    }
}

const MidiEventDataFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_midieventdata_free(ptr >>> 0, 1));

export class MidiEventData {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MidiEventDataFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_midieventdata_free(ptr, 0);
    }
}

const MidiPitchFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_midipitch_free(ptr >>> 0, 1));

export class MidiPitch {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MidiPitchFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_midipitch_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    octave() {
        const ret = wasm.midipitch_octave(this.__wbg_ptr);
        return ret;
    }
}

const ObjectSizesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_objectsizes_free(ptr >>> 0, 1));

export class ObjectSizes {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ObjectSizesFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_objectsizes_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get empty_shape_stroke_width() {
        const ret = wasm.__wbg_get_objectsizes_empty_shape_stroke_width(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set empty_shape_stroke_width(arg0) {
        wasm.__wbg_set_objectsizes_empty_shape_stroke_width(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get small_circle_radius() {
        const ret = wasm.__wbg_get_objectsizes_small_circle_radius(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set small_circle_radius(arg0) {
        wasm.__wbg_set_objectsizes_small_circle_radius(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get dot_radius() {
        const ret = wasm.__wbg_get_objectsizes_dot_radius(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set dot_radius(arg0) {
        wasm.__wbg_set_objectsizes_dot_radius(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get default_line_width() {
        const ret = wasm.__wbg_get_objectsizes_default_line_width(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set default_line_width(arg0) {
        wasm.__wbg_set_objectsizes_default_line_width(this.__wbg_ptr, arg0);
    }
}

const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr >>> 0, 1));

export class Point {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Point.prototype);
        obj.__wbg_ptr = ptr;
        PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get 0() {
        const ret = wasm.__wbg_get_point_0(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set 0(arg0) {
        wasm.__wbg_set_point_0(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get 1() {
        const ret = wasm.__wbg_get_point_1(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set 1(arg0) {
        wasm.__wbg_set_point_1(this.__wbg_ptr, arg0);
    }
}

const RegionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_region_free(ptr >>> 0, 1));

export class Region {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RegionFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_region_free(ptr, 0);
    }
    /**
     * @returns {Point}
     */
    get start() {
        const ret = wasm.__wbg_get_region_start(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set start(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_region_start(this.__wbg_ptr, ptr0);
    }
    /**
     * @returns {Point}
     */
    get end() {
        const ret = wasm.__wbg_get_region_end(this.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * @param {Point} arg0
     */
    set end(arg0) {
        _assertClass(arg0, Point);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_region_end(this.__wbg_ptr, ptr0);
    }
}

const TransformationWASMFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_transformationwasm_free(ptr >>> 0, 1));

export class TransformationWASM {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TransformationWASMFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_transformationwasm_free(ptr, 0);
    }
    /**
     * @returns {TransformationType}
     */
    get kind() {
        const ret = wasm.__wbg_get_transformationwasm_kind(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {TransformationType} arg0
     */
    set kind(arg0) {
        wasm.__wbg_set_transformationwasm_kind(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Float32Array}
     */
    get parameters() {
        const ret = wasm.__wbg_get_transformationwasm_parameters(this.__wbg_ptr);
        var v1 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Float32Array} arg0
     */
    set parameters(arg0) {
        const ptr0 = passArrayF32ToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_transformationwasm_parameters(this.__wbg_ptr, ptr0, len0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_appendChild_7c5825d692053033 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.appendChild(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
        const ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_call_672a4d21634d4a24 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_7cccdd69e0791ae2 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createElement_32c287e69e603e7e = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_crypto_dd1b8f71596b161a = function(arg0) {
        const ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_document_da63b92bac45c6f9 = function(arg0) {
        const ret = arg0.document;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_getRandomValues_760c8e927227643e = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Window_311934805c10047c = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_msCrypto_60a4979188f6b80b = function(arg0) {
        const ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbg_new_a12002a7f91c75be = function(arg0) {
        const ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_newnoargs_105ed471475aaf50 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithlength_a381634e90c276d4 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_node_0deadde112ce24bb = function(arg0) {
        const ret = arg0.node;
        return ret;
    };
    imports.wbg.__wbg_process_0caa4f154b97e834 = function(arg0) {
        const ret = arg0.process;
        return ret;
    };
    imports.wbg.__wbg_querySelector_9a6791de4e8cb055 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.querySelector(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_randomFillSync_82e8b56b81896e30 = function() { return handleError(function (arg0, arg1) {
        arg0.randomFillSync(arg1);
    }, arguments) };
    imports.wbg.__wbg_require_1a22b236558b5786 = function() { return handleError(function () {
        const ret = module.require;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_65595bdd868b3009 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_setclassName_1828bd90d12712ea = function(arg0, arg1, arg2) {
        arg0.className = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setinnerHTML_1f5ad3b02760ea31 = function(arg0, arg1, arg2) {
        arg0.innerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07 = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0 = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_subarray_aa9065fa9dc5df96 = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_versions_134d8f3c6de79566 = function(arg0) {
        const ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(arg0) === 'function';
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('shapemaker_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
