// |reftest| skip -- slow
/* -*- Mode: C++; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* ***** BEGIN LICENSE BLOCK *****
 * Version: MPL 1.1/GPL 2.0/LGPL 2.1
 *
 * The contents of this file are subject to the Mozilla Public License Version
 * 1.1 (the "License"); you may not use this file except in compliance with
 * the License. You may obtain a copy of the License at
 * http://www.mozilla.org/MPL/
 *
 * Software distributed under the License is distributed on an "AS IS" basis,
 * WITHOUT WARRANTY OF ANY KIND, either express or implied. See the License
 * for the specific language governing rights and limitations under the
 * License.
 *
 * The Original Code is JavaScript Engine testing utilities.
 *
 * The Initial Developer of the Original Code is
 * Mozilla Foundation.
 * Portions created by the Initial Developer are Copyright (C) 2006
 * the Initial Developer. All Rights Reserved.
 *
 * Contributor(s): Igor Bukanov
 *
 * Alternatively, the contents of this file may be used under the terms of
 * either the GNU General Public License Version 2 or later (the "GPL"), or
 * the GNU Lesser General Public License Version 2.1 or later (the "LGPL"),
 * in which case the provisions of the GPL or the LGPL are applicable instead
 * of those above. If you wish to allow use of your version of this file only
 * under the terms of either the GPL or the LGPL, and not to allow others to
 * use your version of this file under the terms of the MPL, indicate your
 * decision by deleting the provisions above and replace them with the notice
 * and other provisions required by the GPL or the LGPL. If you do not delete
 * the provisions above, a recipient may use your version of this file under
 * the terms of any one of the MPL, the GPL or the LGPL.
 *
 * ***** END LICENSE BLOCK ***** */

//-----------------------------------------------------------------------------
var BUGNUMBER = 345967;
var summary = 'Yet another unrooted atom in jsarray.c';
var actual = '';
var expect = '';


//-----------------------------------------------------------------------------
test();
//-----------------------------------------------------------------------------

function test()
{
  enterFunc ('test');
  printBugNumber(BUGNUMBER);
  printStatus (summary);

  expectExitCode(0);
  expectExitCode(3);

  print('This test will probably run out of memory');
  print('This test really should only fail on 64 bit machines');
 
  var JSVAL_INT_MAX = (1 << 30) - 1;

  var a = new Array(JSVAL_INT_MAX + 2);
  a[JSVAL_INT_MAX] = 0;
  a[JSVAL_INT_MAX + 1] = 1;

  a.__defineGetter__(JSVAL_INT_MAX, function() { return 0; });

  a.__defineSetter__(JSVAL_INT_MAX, function(value) {
		       delete a[JSVAL_INT_MAX + 1];
		       var tmp = [];
		       tmp[JSVAL_INT_MAX + 2] = 2;

		       if (typeof gc == 'function')
			 gc();
		       for (var i = 0; i != 50000; ++i) {
			 var tmp = 1 / 3;
			 tmp /= 10;
		       }
		       for (var i = 0; i != 1000; ++i) {
			 // Make string with 11 characters that would take
			 // (11 + 1) * 2 bytes or sizeof(JSAtom) so eventually
			 // malloc will ovewrite just freed atoms.
			 var tmp2 = Array(12).join(' ');
		       }
		     });


  a.shift();

  expect = 0;
  actual = a[JSVAL_INT_MAX];
  if (expect !== actual)
    print("BAD");

  reportCompare(expect, actual, summary);

  exitFunc ('test');
}
