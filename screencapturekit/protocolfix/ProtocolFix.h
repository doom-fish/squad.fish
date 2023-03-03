
#import <Foundation/Foundation.h>
#import <ScreenCaptureKit/ScreenCaptureKit.h>

@interface ProtocolFix: NSObject<SCStreamOutput, SCStreamDelegate> {}
@end

