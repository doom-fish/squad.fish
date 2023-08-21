/*
 * Copyright (C) 2010 Ole André Vadla Ravnås <oleavr@soundrop.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */


#include "corevideobuffer.h"
#include "corevideomemory.h"




void
gst_core_video_wrap_pixel_buffer (GstBuffer * buf,
    GstVideoInfo * info,
    CVPixelBufferRef pixel_buf)
{
  guint n_planes;
  gsize offset[GST_VIDEO_MAX_PLANES] = { 0 };
  gint stride[GST_VIDEO_MAX_PLANES] = { 0 };
  UInt32 size;
  GstAppleCoreVideoPixelBuffer *gpixbuf;
  GstMemory *mem = NULL;


  gpixbuf = gst_apple_core_video_pixel_buffer_new (pixel_buf);


  if (CVPixelBufferIsPlanar (pixel_buf)) {
    gint i, size = 0, plane_offset = 0;

    n_planes = CVPixelBufferGetPlaneCount (pixel_buf);
    for (i = 0; i < n_planes; i++) {
      stride[i] = CVPixelBufferGetBytesPerRowOfPlane (pixel_buf, i);


      size = stride[i] * CVPixelBufferGetHeightOfPlane (pixel_buf, i);
      offset[i] = plane_offset;
      plane_offset += size;

      
      mem =
          GST_MEMORY_CAST (gst_apple_core_video_memory_new_wrapped (gpixbuf,
              i, size));
      gst_buffer_append_memory (buf, mem);
    }
  } else {
    n_planes = 1;
    stride[0] = CVPixelBufferGetBytesPerRow (pixel_buf);
    offset[0] = 0;
    size = stride[0] * CVPixelBufferGetHeight (pixel_buf);

  
    mem =
        GST_MEMORY_CAST (gst_apple_core_video_memory_new_wrapped (gpixbuf, 0,
            size));
    gst_buffer_append_memory (buf, mem);
  }

  gst_apple_core_video_pixel_buffer_unref (gpixbuf);

  if (info) {
    gst_buffer_add_video_meta_full (buf, GST_VIDEO_FRAME_FLAG_NONE,
        GST_VIDEO_INFO_FORMAT (info), info->width, info->height, n_planes,
        offset, stride);
  }
}

