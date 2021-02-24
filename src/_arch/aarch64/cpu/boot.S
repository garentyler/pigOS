// SPDX-License-Identifier: MIT
//
// Copyright (c) 2021 Garen Tyler <garentyler@gmail.com>

.section ".text._start"

.global _start

_start:
1:
  wfe // Wait for event
  b 1b // In case an event happened, jump back to 1
