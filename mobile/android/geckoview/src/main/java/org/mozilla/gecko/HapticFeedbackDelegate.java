/* -*- Mode: Java; c-basic-offset: 4; tab-width: 20; indent-tabs-mode: nil; -*-
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.gecko;

import android.view.HapticFeedbackConstants;

/**
 * A <code>HapticFeedbackDelegate</code> is responsible for performing haptic feedback.
 */
public interface HapticFeedbackDelegate {
    /**
     * Perform a haptic feedback effect. Called from the Gecko thread.
     *
     * @param effect Effect to perform from <code>android.view.HapticFeedbackConstants</code>.
     */
    void performHapticFeedback(int effect);
}
