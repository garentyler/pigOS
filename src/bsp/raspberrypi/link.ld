/*  SPDX-License-Identifier: MIT
 *
 * Copyright (c) 2021 Garen Tyler <garentyler@gmail.com>
 */

SECTIONS
{
    /* Set current address to the value from which the RPi starts execution */
    . = 0x80000;

    .text :
    {
        *(.text._start) *(.text*)
    }

    /DISCARD/ : { *(.comment*) }
}
