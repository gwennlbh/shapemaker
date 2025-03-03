/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
* @returns {LayerWeb}
*/
export function new_layer(name: string): LayerWeb;
/**
* @param {Color} c
* @returns {string}
*/
export function color_name(c: Color): string;
/**
* @param {number} opacity
* @param {Color} color
*/
export function render_image(opacity: number, color: Color): void;
/**
*/
export function map_to_midi_controller(): void;
/**
* @param {string} selector
*/
export function render_canvas_into(selector: string): void;
/**
* @param {string} selector
*/
export function render_canvas_at(selector: string): void;
/**
* @returns {string}
*/
export function render_canvas(): string;
/**
* @param {ColorMapping} palette
*/
export function set_palette(palette: ColorMapping): void;
/**
* @param {string} name
* @returns {LayerWeb}
*/
export function get_layer(name: string): LayerWeb;
/**
* @param {string} name
* @returns {LayerWeb}
*/
export function random_linelikes(name: string): LayerWeb;
/**
* @param {Color | undefined} [except]
* @returns {Color}
*/
export function random_color(except?: Color): Color;
/**
* @param {string} s
* @returns {string}
*/
export function slugify(s: string): string;
/**
*/
export enum TransformationType {
  Scale = 0,
  Rotate = 1,
  Skew = 2,
  Matrix = 3,
}
/**
*/
export enum FilterType {
  Glow = 0,
  NaturalShadow = 1,
  Saturation = 2,
}
/**
*/
export enum Color {
  Black = 0,
  White = 1,
  Red = 2,
  Green = 3,
  Blue = 4,
  Yellow = 5,
  Orange = 6,
  Purple = 7,
  Brown = 8,
  Cyan = 9,
  Pink = 10,
  Gray = 11,
}
/**
*/
export enum MidiEvent {
  Note = 0,
  ControlChange = 1,
}
/**
*/
export class ColorMapping {
  free(): void;
/**
* @returns {ColorMapping}
*/
  static default(): ColorMapping;
/**
* @param {string} content
* @returns {ColorMapping}
*/
  static from_json(content: string): ColorMapping;
/**
* @param {string} content
* @returns {ColorMapping}
*/
  static from_css(content: string): ColorMapping;
/**
*/
  black: string;
/**
*/
  blue: string;
/**
*/
  brown: string;
/**
*/
  cyan: string;
/**
*/
  gray: string;
/**
*/
  green: string;
/**
*/
  orange: string;
/**
*/
  pink: string;
/**
*/
  purple: string;
/**
*/
  red: string;
/**
*/
  white: string;
/**
*/
  yellow: string;
}
/**
*/
export class Filter {
  free(): void;
/**
* @returns {string}
*/
  name(): string;
/**
* @param {number} intensity
* @returns {Filter}
*/
  static glow(intensity: number): Filter;
/**
* @returns {string}
*/
  id(): string;
/**
*/
  kind: FilterType;
/**
*/
  parameter: number;
}
/**
*/
export class LayerWeb {
  free(): void;
/**
* @returns {string}
*/
  render(): string;
/**
* @param {string} selector
*/
  render_into(selector: string): void;
/**
* @param {string} selector
*/
  render_at(selector: string): void;
/**
* @param {Color} color
* @param {number | undefined} opacity
* @param {Filter} filter
*/
  paint_all(color: Color, opacity: number | undefined, filter: Filter): void;
/**
* @param {string} name
* @returns {LayerWeb}
*/
  static random(name: string): LayerWeb;
/**
* @param {string} name
* @param {Point} start
* @param {Point} end
* @param {number} thickness
* @param {Color} color
*/
  new_line(name: string, start: Point, end: Point, thickness: number, color: Color): void;
/**
* @param {string} name
* @param {Point} start
* @param {Point} end
* @param {number} thickness
* @param {Color} color
*/
  new_curve_outward(name: string, start: Point, end: Point, thickness: number, color: Color): void;
/**
* @param {string} name
* @param {Point} start
* @param {Point} end
* @param {number} thickness
* @param {Color} color
*/
  new_curve_inward(name: string, start: Point, end: Point, thickness: number, color: Color): void;
/**
* @param {string} name
* @param {Point} center
* @param {Color} color
*/
  new_small_circle(name: string, center: Point, color: Color): void;
/**
* @param {string} name
* @param {Point} center
* @param {Color} color
*/
  new_dot(name: string, center: Point, color: Color): void;
/**
* @param {string} name
* @param {Point} center
* @param {Color} color
*/
  new_big_circle(name: string, center: Point, color: Color): void;
/**
* @param {string} name
* @param {Point} anchor
* @param {string} text
* @param {number} font_size
* @param {Color} color
*/
  new_text(name: string, anchor: Point, text: string, font_size: number, color: Color): void;
/**
* @param {string} name
* @param {Point} topleft
* @param {Point} bottomright
* @param {Color} color
*/
  new_rectangle(name: string, topleft: Point, bottomright: Point, color: Color): void;
/**
*/
  name: string;
}
/**
*/
export class MidiEventData {
  free(): void;
}
/**
*/
export class MidiPitch {
  free(): void;
/**
* @returns {number}
*/
  octave(): number;
}
/**
*/
export class ObjectSizes {
  free(): void;
/**
*/
  default_line_width: number;
/**
*/
  dot_radius: number;
/**
*/
  empty_shape_stroke_width: number;
/**
*/
  small_circle_radius: number;
}
/**
*/
export class Point {
  free(): void;
/**
*/
  0: number;
/**
*/
  1: number;
}
/**
*/
export class Region {
  free(): void;
/**
*/
  end: Point;
/**
*/
  start: Point;
}
/**
*/
export class TransformationWASM {
  free(): void;
/**
*/
  kind: TransformationType;
/**
*/
  parameters: Float32Array;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_point_free: (a: number) => void;
  readonly __wbg_get_point_0: (a: number) => number;
  readonly __wbg_set_point_0: (a: number, b: number) => void;
  readonly __wbg_get_point_1: (a: number) => number;
  readonly __wbg_set_point_1: (a: number, b: number) => void;
  readonly __wbg_region_free: (a: number) => void;
  readonly __wbg_get_region_start: (a: number) => number;
  readonly __wbg_set_region_start: (a: number, b: number) => void;
  readonly __wbg_get_region_end: (a: number) => number;
  readonly __wbg_set_region_end: (a: number, b: number) => void;
  readonly __wbg_colormapping_free: (a: number) => void;
  readonly __wbg_get_colormapping_black: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_black: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_white: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_white: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_red: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_red: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_green: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_green: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_blue: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_blue: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_yellow: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_yellow: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_orange: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_orange: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_purple: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_purple: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_brown: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_brown: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_cyan: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_cyan: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_pink: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_pink: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_gray: (a: number, b: number) => void;
  readonly __wbg_set_colormapping_gray: (a: number, b: number, c: number) => void;
  readonly colormapping_default: () => number;
  readonly colormapping_from_json: (a: number, b: number) => number;
  readonly colormapping_from_css: (a: number, b: number) => number;
  readonly __wbg_layerweb_free: (a: number) => void;
  readonly new_layer: (a: number, b: number) => number;
  readonly layerweb_render: (a: number, b: number) => void;
  readonly layerweb_render_into: (a: number, b: number, c: number) => void;
  readonly layerweb_render_at: (a: number, b: number, c: number) => void;
  readonly layerweb_paint_all: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layerweb_random: (a: number, b: number) => number;
  readonly layerweb_new_line: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly layerweb_new_curve_outward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly layerweb_new_curve_inward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly layerweb_new_small_circle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layerweb_new_dot: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layerweb_new_big_circle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layerweb_new_text: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly layerweb_new_rectangle: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_set_layerweb_name: (a: number, b: number, c: number) => void;
  readonly __wbg_get_layerweb_name: (a: number, b: number) => void;
  readonly color_name: (a: number, b: number) => void;
  readonly render_image: (a: number, b: number, c: number) => void;
  readonly map_to_midi_controller: () => void;
  readonly render_canvas_into: (a: number, b: number) => void;
  readonly render_canvas_at: (a: number, b: number) => void;
  readonly __wbg_midieventdata_free: (a: number) => void;
  readonly midipitch_octave: (a: number) => number;
  readonly render_canvas: (a: number) => void;
  readonly set_palette: (a: number) => void;
  readonly get_layer: (a: number, b: number, c: number) => void;
  readonly random_linelikes: (a: number, b: number) => number;
  readonly __wbg_midipitch_free: (a: number) => void;
  readonly __wbg_filter_free: (a: number) => void;
  readonly __wbg_get_filter_kind: (a: number) => number;
  readonly __wbg_set_filter_kind: (a: number, b: number) => void;
  readonly __wbg_get_filter_parameter: (a: number) => number;
  readonly __wbg_set_filter_parameter: (a: number, b: number) => void;
  readonly filter_name: (a: number, b: number) => void;
  readonly filter_glow: (a: number) => number;
  readonly filter_id: (a: number, b: number) => void;
  readonly random_color: (a: number) => number;
  readonly __wbg_objectsizes_free: (a: number) => void;
  readonly __wbg_get_objectsizes_empty_shape_stroke_width: (a: number) => number;
  readonly __wbg_set_objectsizes_empty_shape_stroke_width: (a: number, b: number) => void;
  readonly __wbg_get_objectsizes_small_circle_radius: (a: number) => number;
  readonly __wbg_set_objectsizes_small_circle_radius: (a: number, b: number) => void;
  readonly __wbg_get_objectsizes_dot_radius: (a: number) => number;
  readonly __wbg_set_objectsizes_dot_radius: (a: number, b: number) => void;
  readonly __wbg_get_objectsizes_default_line_width: (a: number) => number;
  readonly __wbg_set_objectsizes_default_line_width: (a: number, b: number) => void;
  readonly __wbg_transformationwasm_free: (a: number) => void;
  readonly __wbg_get_transformationwasm_kind: (a: number) => number;
  readonly __wbg_set_transformationwasm_kind: (a: number, b: number) => void;
  readonly __wbg_get_transformationwasm_parameters: (a: number, b: number) => void;
  readonly __wbg_set_transformationwasm_parameters: (a: number, b: number, c: number) => void;
  readonly slugify: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
