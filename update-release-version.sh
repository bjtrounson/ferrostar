#!/usr/bin/env zsh

set -e
set -u

version=$1

# Swift
sed -i "" -E "s/(let releaseTag = \")[^\"]+(\")/\1$version\2/g" Package.swift

# Android
sed -i "" -E "s/(version = \")[^\"]+(\")/\1$version\2/g" android/build.gradle

# Rust
awk '{ if (!done && /version = \"/) { sub(/(version = \")[^\"]+(\")/, "version = \"" newVersion "\""); done=1 } print }' newVersion="$version" common/ferrostar/Cargo.toml > tmpfile && mv tmpfile common/ferrostar/Cargo.toml
cd common && cargo check && cd ..

# Web components
sed -i "" -E "s/(\"version\": \")[^\"]+(\")/\1$version\2/g" web/package.json
cd web && npm install && cd ..

# React Native
sed -i "" -E "s/(\"version\": \")[^\"]+(\")/\1$version\2/g" react-native/package.json
sed -i "" -E "s/(\"version\": \")[^\"]+(\")/\1$version\2/g" react-native/core/package.json
sed -i "" -E "s/(\"version\": \")[^\"]+(\")/\1$version\2/g" react-native/uniffi/package.json
sed -i "" -E "s/(\"version\": \")[^\"]+(\")/\1$version\2/g" react-native/maplibreui/package.json
cd react-native && yarn install && cd ..

git add Package.swift android/build.gradle common/Cargo.lock common/ferrostar/Cargo.toml web/package.json web/package-lock.json react-native/package.json react-native/yarn.lock react-native/core/package.json react-native/uniffi/package.json react-native/maplibreui/package.json
