// This file was autogenerated by some hot garbage in the `uniffi-bindgen-react-native` crate.
// Trust me, you don't want to mess with it!

import {
  type StructuralEquality as UniffiStructuralEquality,
  type UniffiReferenceHolder,
  type UniffiRustArcPtr,
  type UniffiRustCallStatus,
  type UniffiRustFutureContinuationCallback as RuntimeUniffiRustFutureContinuationCallback,
} from 'uniffi-bindgen-react-native';

interface NativeModuleInterface {
  uniffi_internal_fn_func_ffi__string_to_byte_length(
    string: string,
    uniffi_out_err: UniffiRustCallStatus
  ): number;
  uniffi_internal_fn_func_ffi__string_to_arraybuffer(
    string: string,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_internal_fn_func_ffi__arraybuffer_to_string(
    buffer: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): string;
  uniffi_ferrostar_fn_clone_navigationcontroller(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_free_navigationcontroller(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): void;
  uniffi_ferrostar_fn_constructor_navigationcontroller_new(
    route: ArrayBuffer,
    config: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_method_navigationcontroller_advance_to_next_step(
    ptr: bigint,
    state: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_method_navigationcontroller_get_initial_state(
    ptr: bigint,
    location: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_method_navigationcontroller_update_user_location(
    ptr: bigint,
    location: ArrayBuffer,
    state: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_clone_routeadapter(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_free_routeadapter(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): void;
  uniffi_ferrostar_fn_constructor_routeadapter_new(
    requestGenerator: bigint,
    responseParser: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_constructor_routeadapter_new_valhalla_http(
    endpointUrl: ArrayBuffer,
    profile: ArrayBuffer,
    optionsJson: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_method_routeadapter_generate_request(
    ptr: bigint,
    userLocation: ArrayBuffer,
    waypoints: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_method_routeadapter_parse_response(
    ptr: bigint,
    response: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_clone_routedeviationdetector(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_free_routedeviationdetector(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): void;
  uniffi_ferrostar_fn_method_routedeviationdetector_check_route_deviation(
    ptr: bigint,
    location: ArrayBuffer,
    route: ArrayBuffer,
    currentRouteStep: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_clone_routerequestgenerator(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_free_routerequestgenerator(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): void;
  uniffi_ferrostar_fn_method_routerequestgenerator_generate_request(
    ptr: bigint,
    userLocation: ArrayBuffer,
    waypoints: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_clone_routeresponseparser(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_free_routeresponseparser(
    ptr: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): void;
  uniffi_ferrostar_fn_method_routeresponseparser_parse_response(
    ptr: bigint,
    response: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_func_advance_location_simulation(
    state: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_func_create_ferrostar_logger(
    uniffi_out_err: UniffiRustCallStatus
  ): void;
  uniffi_ferrostar_fn_func_create_osrm_response_parser(
    polylinePrecision: number,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_func_create_route_from_osrm(
    routeData: ArrayBuffer,
    waypointData: ArrayBuffer,
    polylinePrecision: number,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_func_create_valhalla_request_generator(
    endpointUrl: ArrayBuffer,
    profile: ArrayBuffer,
    optionsJson: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): bigint;
  uniffi_ferrostar_fn_func_get_route_polyline(
    route: ArrayBuffer,
    precision: number,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_func_location_simulation_from_coordinates(
    coordinates: ArrayBuffer,
    resampleDistance: ArrayBuffer,
    bias: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_func_location_simulation_from_polyline(
    polyline: ArrayBuffer,
    precision: number,
    resampleDistance: ArrayBuffer,
    bias: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_fn_func_location_simulation_from_route(
    route: ArrayBuffer,
    resampleDistance: ArrayBuffer,
    bias: ArrayBuffer,
    uniffi_out_err: UniffiRustCallStatus
  ): ArrayBuffer;
  uniffi_ferrostar_checksum_func_advance_location_simulation(): number;
  uniffi_ferrostar_checksum_func_create_ferrostar_logger(): number;
  uniffi_ferrostar_checksum_func_create_osrm_response_parser(): number;
  uniffi_ferrostar_checksum_func_create_route_from_osrm(): number;
  uniffi_ferrostar_checksum_func_create_valhalla_request_generator(): number;
  uniffi_ferrostar_checksum_func_get_route_polyline(): number;
  uniffi_ferrostar_checksum_func_location_simulation_from_coordinates(): number;
  uniffi_ferrostar_checksum_func_location_simulation_from_polyline(): number;
  uniffi_ferrostar_checksum_func_location_simulation_from_route(): number;
  uniffi_ferrostar_checksum_method_navigationcontroller_advance_to_next_step(): number;
  uniffi_ferrostar_checksum_method_navigationcontroller_get_initial_state(): number;
  uniffi_ferrostar_checksum_method_navigationcontroller_update_user_location(): number;
  uniffi_ferrostar_checksum_method_routeadapter_generate_request(): number;
  uniffi_ferrostar_checksum_method_routeadapter_parse_response(): number;
  uniffi_ferrostar_checksum_method_routedeviationdetector_check_route_deviation(): number;
  uniffi_ferrostar_checksum_method_routerequestgenerator_generate_request(): number;
  uniffi_ferrostar_checksum_method_routeresponseparser_parse_response(): number;
  uniffi_ferrostar_checksum_constructor_navigationcontroller_new(): number;
  uniffi_ferrostar_checksum_constructor_routeadapter_new(): number;
  uniffi_ferrostar_checksum_constructor_routeadapter_new_valhalla_http(): number;
  ffi_ferrostar_uniffi_contract_version(): number;
  uniffi_ferrostar_fn_init_callback_vtable_routedeviationdetector(
    vtable: UniffiVTableCallbackInterfaceRouteDeviationDetector
  ): void;
  uniffi_ferrostar_fn_init_callback_vtable_routerequestgenerator(
    vtable: UniffiVTableCallbackInterfaceRouteRequestGenerator
  ): void;
  uniffi_ferrostar_fn_init_callback_vtable_routeresponseparser(
    vtable: UniffiVTableCallbackInterfaceRouteResponseParser
  ): void;
  uniffi_internal_fn_method_navigationcontroller_ffi__bless_pointer(
    pointer: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): UniffiRustArcPtr;
  uniffi_internal_fn_method_routeadapter_ffi__bless_pointer(
    pointer: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): UniffiRustArcPtr;
  uniffi_internal_fn_method_routedeviationdetector_ffi__bless_pointer(
    pointer: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): UniffiRustArcPtr;
  uniffi_internal_fn_method_routerequestgenerator_ffi__bless_pointer(
    pointer: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): UniffiRustArcPtr;
  uniffi_internal_fn_method_routeresponseparser_ffi__bless_pointer(
    pointer: bigint,
    uniffi_out_err: UniffiRustCallStatus
  ): UniffiRustArcPtr;
}

// Casting globalThis to any allows us to look for `NativeFerrostar`
// if it was added via JSI.
//
// We use a getter here rather than simply `globalThis.NativeFerrostar` so that
// if/when the startup sequence isn't just so, an empty value isn't inadvertantly cached.
const getter: () => NativeModuleInterface = () =>
  (globalThis as any).NativeFerrostar;
export default getter;

// Structs and function types for calling back into Typescript from Rust.
export type UniffiRustFutureContinuationCallback = (
  data: bigint,
  pollResult: number
) => void;
export type UniffiForeignFutureFree = (handle: bigint) => void;
export type UniffiCallbackInterfaceFree = (handle: bigint) => void;
export type UniffiForeignFuture = {
  handle: bigint;
  free: UniffiForeignFutureFree;
};
export type UniffiForeignFutureStructU8 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteU8 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructU8
) => void;
export type UniffiForeignFutureStructI8 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteI8 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructI8
) => void;
export type UniffiForeignFutureStructU16 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteU16 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructU16
) => void;
export type UniffiForeignFutureStructI16 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteI16 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructI16
) => void;
export type UniffiForeignFutureStructU32 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteU32 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructU32
) => void;
export type UniffiForeignFutureStructI32 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteI32 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructI32
) => void;
export type UniffiForeignFutureStructU64 = {
  returnValue: bigint;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteU64 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructU64
) => void;
export type UniffiForeignFutureStructI64 = {
  returnValue: bigint;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteI64 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructI64
) => void;
export type UniffiForeignFutureStructF32 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteF32 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructF32
) => void;
export type UniffiForeignFutureStructF64 = {
  returnValue: number;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteF64 = (
  callbackData: bigint,
  result: UniffiForeignFutureStructF64
) => void;
export type UniffiForeignFutureStructPointer = {
  returnValue: bigint;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompletePointer = (
  callbackData: bigint,
  result: UniffiForeignFutureStructPointer
) => void;
export type UniffiForeignFutureStructRustBuffer = {
  returnValue: ArrayBuffer;
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteRustBuffer = (
  callbackData: bigint,
  result: UniffiForeignFutureStructRustBuffer
) => void;
export type UniffiForeignFutureStructVoid = {
  callStatus: UniffiRustCallStatus;
};
export type UniffiForeignFutureCompleteVoid = (
  callbackData: bigint,
  result: UniffiForeignFutureStructVoid
) => void;
type UniffiCallbackInterfaceRouteDeviationDetectorMethod0 = (
  uniffiHandle: bigint,
  location: ArrayBuffer,
  route: ArrayBuffer,
  currentRouteStep: ArrayBuffer,
  uniffiOutReturn: UniffiReferenceHolder<ArrayBuffer>,
  callStatus: UniffiRustCallStatus
) => void;
type UniffiCallbackInterfaceRouteRequestGeneratorMethod0 = (
  uniffiHandle: bigint,
  userLocation: ArrayBuffer,
  waypoints: ArrayBuffer,
  uniffiOutReturn: UniffiReferenceHolder<ArrayBuffer>,
  callStatus: UniffiRustCallStatus
) => void;
type UniffiCallbackInterfaceRouteResponseParserMethod0 = (
  uniffiHandle: bigint,
  response: ArrayBuffer,
  uniffiOutReturn: UniffiReferenceHolder<ArrayBuffer>,
  callStatus: UniffiRustCallStatus
) => void;
export type UniffiVTableCallbackInterfaceRouteDeviationDetector = {
  checkRouteDeviation: UniffiCallbackInterfaceRouteDeviationDetectorMethod0;
  uniffiFree: UniffiCallbackInterfaceFree;
};
export type UniffiVTableCallbackInterfaceRouteRequestGenerator = {
  generateRequest: UniffiCallbackInterfaceRouteRequestGeneratorMethod0;
  uniffiFree: UniffiCallbackInterfaceFree;
};
export type UniffiVTableCallbackInterfaceRouteResponseParser = {
  parseResponse: UniffiCallbackInterfaceRouteResponseParserMethod0;
  uniffiFree: UniffiCallbackInterfaceFree;
};

// UniffiRustFutureContinuationCallback is generated as part of the component interface's
// ffi_definitions. However, we need it in the runtime.
// We could:
// (a) do some complicated template logic to ensure the declaration is not generated here (possible)
// (b) import the generated declaration into the runtime (m a y b e) or…
// (c) generate the declaration anyway, and use a different declaration in the runtime.
//
// We chose (c) here as the simplest. In addition, we perform a compile time check that
// the two versions of `UniffiRustFutureContinuationCallback` are structurally equivalent.
//
// If you see the error:
// ```
// Type 'true' is not assignable to type 'false'.(2322)
// ```
// Then a new version of uniffi has changed the signature of the callback. Most likely, code in
// `typescript/src/async-rust-call.ts` will need to be changed.
//
// If you see the error:
// ```
// Cannot find name 'UniffiRustFutureContinuationCallback'. Did you mean 'RuntimeUniffiRustFutureContinuationCallback'?(2552)
// ```
// then you may not be using callbacks or promises, and uniffi is now not generating Futures and callbacks.
// You should not generate this if that is the case.
//
// ('You' being the bindings generator maintainer).
const isRustFutureContinuationCallbackTypeCompatible: UniffiStructuralEquality<
  RuntimeUniffiRustFutureContinuationCallback,
  UniffiRustFutureContinuationCallback
> = true;