/* tslint:disable */
/* eslint-disable */
export function new_layer(name: string): LayerWeb;
export function color_name(c: Color): string;
export function render_image(opacity: number, color: Color): void;
export function map_to_midi_controller(): void;
export function render_canvas_into(selector: string): void;
export function render_canvas_at(selector: string): void;
export function render_canvas(): string;
export function set_palette(palette: ColorMapping): void;
export function get_layer(name: string): LayerWeb;
export function random_linelikes(name: string): LayerWeb;
export function slugify(s: string): string;
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
export enum FilterType {
  Glow = 0,
  NaturalShadow = 1,
  Saturation = 2,
}
export enum MidiEvent {
  Note = 0,
  ControlChange = 1,
}
export enum TransformationType {
  Scale = 0,
  Rotate = 1,
  Skew = 2,
  Matrix = 3,
}
export class ColorMapping {
  private constructor();
  free(): void;
  static default(): ColorMapping;
  static from_json(content: string): ColorMapping;
  static from_css(content: string): ColorMapping;
  black: string;
  white: string;
  red: string;
  green: string;
  blue: string;
  yellow: string;
  orange: string;
  purple: string;
  brown: string;
  cyan: string;
  pink: string;
  gray: string;
}
export class Filter {
  private constructor();
  free(): void;
  name(): string;
  static glow(intensity: number): Filter;
  id(): string;
  kind: FilterType;
  parameter: number;
}
export class LayerWeb {
  private constructor();
  free(): void;
  render(): string;
  render_into(selector: string): void;
  render_at(selector: string): void;
  paint_all(color: Color, opacity: number | null | undefined, filter: Filter): void;
  static random(name: string): LayerWeb;
  new_line(name: string, start: Point, end: Point, thickness: number, color: Color): void;
  new_curve_outward(name: string, start: Point, end: Point, thickness: number, color: Color): void;
  new_curve_inward(name: string, start: Point, end: Point, thickness: number, color: Color): void;
  new_small_circle(name: string, center: Point, color: Color): void;
  new_dot(name: string, center: Point, color: Color): void;
  new_big_circle(name: string, center: Point, color: Color): void;
  new_text(name: string, anchor: Point, text: string, font_size: number, color: Color): void;
  new_rectangle(name: string, topleft: Point, bottomright: Point, color: Color): void;
  name: string;
}
export class MidiEventData {
  private constructor();
  free(): void;
}
export class MidiPitch {
  private constructor();
  free(): void;
  octave(): number;
}
export class ObjectSizes {
  private constructor();
  free(): void;
  empty_shape_stroke_width: number;
  small_circle_radius: number;
  dot_radius: number;
  default_line_width: number;
}
export class Point {
  private constructor();
  free(): void;
  0: number;
  1: number;
}
export class Region {
  private constructor();
  free(): void;
  start: Point;
  end: Point;
}
export class TransformationWASM {
  private constructor();
  free(): void;
  kind: TransformationType;
  parameters: Float32Array;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_point_free: (a: number, b: number) => void;
  readonly __wbg_get_point_0: (a: number) => number;
  readonly __wbg_set_point_0: (a: number, b: number) => void;
  readonly __wbg_get_point_1: (a: number) => number;
  readonly __wbg_set_point_1: (a: number, b: number) => void;
  readonly __wbg_filter_free: (a: number, b: number) => void;
  readonly __wbg_get_filter_kind: (a: number) => number;
  readonly __wbg_set_filter_kind: (a: number, b: number) => void;
  readonly __wbg_get_filter_parameter: (a: number) => number;
  readonly __wbg_set_filter_parameter: (a: number, b: number) => void;
  readonly filter_name: (a: number) => [number, number];
  readonly filter_glow: (a: number) => number;
  readonly filter_id: (a: number) => [number, number];
  readonly __wbg_layerweb_free: (a: number, b: number) => void;
  readonly __wbg_get_layerweb_name: (a: number) => [number, number];
  readonly __wbg_set_layerweb_name: (a: number, b: number, c: number) => void;
  readonly new_layer: (a: number, b: number) => number;
  readonly layerweb_render: (a: number) => [number, number];
  readonly layerweb_render_into: (a: number, b: number, c: number) => void;
  readonly layerweb_render_at: (a: number, b: number, c: number) => void;
  readonly layerweb_paint_all: (a: number, b: number, c: number, d: number) => void;
  readonly layerweb_random: (a: number, b: number) => number;
  readonly layerweb_new_line: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly layerweb_new_curve_outward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly layerweb_new_curve_inward: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly layerweb_new_small_circle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layerweb_new_dot: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layerweb_new_big_circle: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly layerweb_new_text: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly layerweb_new_rectangle: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_region_free: (a: number, b: number) => void;
  readonly __wbg_get_region_start: (a: number) => number;
  readonly __wbg_set_region_start: (a: number, b: number) => void;
  readonly __wbg_get_region_end: (a: number) => number;
  readonly __wbg_set_region_end: (a: number, b: number) => void;
  readonly color_name: (a: number) => [number, number];
  readonly render_image: (a: number, b: number) => [number, number];
  readonly map_to_midi_controller: () => void;
  readonly render_canvas_into: (a: number, b: number) => void;
  readonly render_canvas_at: (a: number, b: number) => void;
  readonly __wbg_midieventdata_free: (a: number, b: number) => void;
  readonly __wbg_midipitch_free: (a: number, b: number) => void;
  readonly midipitch_octave: (a: number) => number;
  readonly render_canvas: () => [number, number];
  readonly set_palette: (a: number) => void;
  readonly get_layer: (a: number, b: number) => [number, number, number];
  readonly random_linelikes: (a: number, b: number) => number;
  readonly __wbg_objectsizes_free: (a: number, b: number) => void;
  readonly __wbg_get_objectsizes_empty_shape_stroke_width: (a: number) => number;
  readonly __wbg_set_objectsizes_empty_shape_stroke_width: (a: number, b: number) => void;
  readonly __wbg_get_objectsizes_small_circle_radius: (a: number) => number;
  readonly __wbg_set_objectsizes_small_circle_radius: (a: number, b: number) => void;
  readonly __wbg_get_objectsizes_dot_radius: (a: number) => number;
  readonly __wbg_set_objectsizes_dot_radius: (a: number, b: number) => void;
  readonly __wbg_get_objectsizes_default_line_width: (a: number) => number;
  readonly __wbg_set_objectsizes_default_line_width: (a: number, b: number) => void;
  readonly __wbg_transformationwasm_free: (a: number, b: number) => void;
  readonly __wbg_get_transformationwasm_kind: (a: number) => number;
  readonly __wbg_set_transformationwasm_kind: (a: number, b: number) => void;
  readonly __wbg_get_transformationwasm_parameters: (a: number) => [number, number];
  readonly __wbg_set_transformationwasm_parameters: (a: number, b: number, c: number) => void;
  readonly __wbg_colormapping_free: (a: number, b: number) => void;
  readonly __wbg_get_colormapping_black: (a: number) => [number, number];
  readonly __wbg_set_colormapping_black: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_white: (a: number) => [number, number];
  readonly __wbg_set_colormapping_white: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_red: (a: number) => [number, number];
  readonly __wbg_set_colormapping_red: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_green: (a: number) => [number, number];
  readonly __wbg_set_colormapping_green: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_blue: (a: number) => [number, number];
  readonly __wbg_set_colormapping_blue: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_yellow: (a: number) => [number, number];
  readonly __wbg_set_colormapping_yellow: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_orange: (a: number) => [number, number];
  readonly __wbg_set_colormapping_orange: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_purple: (a: number) => [number, number];
  readonly __wbg_set_colormapping_purple: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_brown: (a: number) => [number, number];
  readonly __wbg_set_colormapping_brown: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_cyan: (a: number) => [number, number];
  readonly __wbg_set_colormapping_cyan: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_pink: (a: number) => [number, number];
  readonly __wbg_set_colormapping_pink: (a: number, b: number, c: number) => void;
  readonly __wbg_get_colormapping_gray: (a: number) => [number, number];
  readonly __wbg_set_colormapping_gray: (a: number, b: number, c: number) => void;
  readonly colormapping_default: () => number;
  readonly colormapping_from_json: (a: number, b: number) => number;
  readonly colormapping_from_css: (a: number, b: number) => number;
  readonly slugify: (a: number, b: number) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
