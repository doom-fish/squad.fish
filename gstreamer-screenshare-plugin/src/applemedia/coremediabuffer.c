/*
 * Copyright (C) 2009 Ole André Vadla Ravnås <oleavr@soundrop.com>
 * Copyright (C) 2014 Collabora Ltd.
 *   Authors:    Matthieu Bouron <matthieu.bouron@collabora.com>
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

#include <objc/runtime.h>
#include <objc/message.h>
#include "coremediabuffer.h"
#include "corevideobuffer.h"
#include "corevideomemory.h"
static const GstMetaInfo *gst_core_media_meta_get_info(void);

static void gst_core_media_meta_add(GstBuffer *buffer,
                                    CVImageBufferRef image_buf) {
  
  GstCoreMediaMeta *meta;
  meta = (GstCoreMediaMeta *)gst_buffer_add_meta(
      buffer, gst_core_media_meta_get_info(), NULL);
  meta->image_buf = image_buf;
  if (image_buf != NULL && CFGetTypeID(image_buf) == CVPixelBufferGetTypeID()) {

    meta->pixel_buf = (CVPixelBufferRef)image_buf;
  }
  else
    meta->pixel_buf = NULL;
}

static gboolean gst_core_media_meta_init(GstCoreMediaMeta *meta,
                                         gpointer params, GstBuffer *buf) {
  meta->sample_buf = NULL;
  meta->image_buf = NULL;
  meta->pixel_buf = NULL;
  meta->block_buf = NULL;

  return TRUE;
}

static void gst_core_media_meta_free(GstCoreMediaMeta *meta, GstBuffer *buf) {
  if (meta->image_buf != NULL) {
    CVBufferRelease(meta->image_buf);
  }
}

static gboolean gst_core_media_meta_transform(GstBuffer *transbuf,
                                              GstCoreMediaMeta *meta,
                                              GstBuffer *buffer, GQuark type,
                                              GstMetaTransformCopy *data) {
  if (!data->region) {
    /* only copy if the complete data is copied as well */
    gst_core_media_meta_add(transbuf, meta->image_buf);
  } else {
    GST_WARNING_OBJECT(transbuf,
                       "dropping Core Media metadata due to partial buffer");
  }

  return TRUE; /* retval unused */
}

GType gst_core_media_meta_api_get_type(void) {
  static GType type;
  static const gchar *tags[] = {"memory", NULL};

  if (g_once_init_enter(&type)) {
    GType _type = gst_meta_api_type_register("GstCoreMediaMetaAPI", tags);
    g_once_init_leave(&type, _type);
  }
  return type;
}

static const GstMetaInfo *gst_core_media_meta_get_info(void) {
  static const GstMetaInfo *core_media_meta_info = NULL;

  if (g_once_init_enter(&core_media_meta_info)) {
    const GstMetaInfo *meta = gst_meta_register(
        GST_CORE_MEDIA_META_API_TYPE, "GstCoreMediaMeta",
        sizeof(GstCoreMediaMeta), (GstMetaInitFunction)gst_core_media_meta_init,
        (GstMetaFreeFunction)gst_core_media_meta_free,
        (GstMetaTransformFunction)gst_core_media_meta_transform);
    g_once_init_leave(&core_media_meta_info, meta);
  }
  return core_media_meta_info;
}

static GstVideoFormat gst_core_media_buffer_get_video_format(OSType format) {
  switch (format) {
  case kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange:
  case kCVPixelFormatType_420YpCbCr8BiPlanarFullRange:
    return GST_VIDEO_FORMAT_NV12;
  case kCVPixelFormatType_422YpCbCr8_yuvs:
    return GST_VIDEO_FORMAT_YUY2;
  case kCVPixelFormatType_422YpCbCr8:
    return GST_VIDEO_FORMAT_UYVY;
  case kCVPixelFormatType_32BGRA:
    return GST_VIDEO_FORMAT_BGRA;
  case kCVPixelFormatType_32RGBA:
    return GST_VIDEO_FORMAT_RGBA;
  default:
    GST_WARNING("Unknown OSType format: %d", (gint)format);
    return GST_VIDEO_FORMAT_UNKNOWN;
  }
}

static gboolean
gst_video_info_init_from_pixel_buffer(GstVideoInfo *info,
                                      CVPixelBufferRef pixel_buf) {
  size_t width, height;
  OSType format_type;
  GstVideoFormat video_format;

  width = CVPixelBufferGetWidth(pixel_buf);
  height = CVPixelBufferGetHeight(pixel_buf);
  format_type = CVPixelBufferGetPixelFormatType(pixel_buf);
  video_format = gst_core_media_buffer_get_video_format(format_type);
  if (video_format == GST_VIDEO_FORMAT_UNKNOWN) {
    return FALSE;
  }

  gst_video_info_init(info);
  gst_video_info_set_format(info, video_format, width, height);

  return TRUE;
}

GstBuffer *gst_core_media_buffer_new(CMSampleBufferRef sample_buf)
{


  GstBuffer *buf;
  CVImageBufferRef image_buf = CMSampleBufferGetImageBuffer(sample_buf);
  
  buf = gst_buffer_new();

  gst_core_media_meta_add(buf, image_buf);
  if (image_buf != NULL && CFGetTypeID(image_buf) == CVPixelBufferGetTypeID()) {
    GstVideoInfo info;
    CVPixelBufferRef pixel_buf = (CVPixelBufferRef)image_buf;
    if (!gst_video_info_init_from_pixel_buffer(&info, pixel_buf)) {
      goto error;
    }
    gst_core_video_wrap_pixel_buffer(buf, &info, pixel_buf);
  } else {
    goto error;
  }

  return buf;

error:
  if (buf) {
    gst_buffer_unref(buf);
  }
  
  return NULL;
}
