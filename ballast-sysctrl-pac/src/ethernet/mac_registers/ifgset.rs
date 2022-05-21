#[doc = "Register `IFGset` reader"]
pub struct R(crate::R<IFGSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFGSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFGSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFGSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFGset` writer"]
pub struct W(crate::W<IFGSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFGSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IFGSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFGSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFGset` reader - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub struct IFGSET_R(crate::FieldReader<u8, u8>);
impl IFGSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IFGSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFGSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFGset` writer - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
pub struct IFGSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IFGSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    #[inline(always)]
    pub fn ifgset(&self) -> IFGSET_R {
        IFGSET_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame."]
    #[inline(always)]
    pub fn ifgset(&mut self) -> IFGSET_W {
        IFGSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX_IFG_SET is used to set received frame gap. If the gap between two received packets is less than RX_IFG_SET,the second packet will be drop as an invalid frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifgset](index.html) module"]
pub struct IFGSET_SPEC;
impl crate::RegisterSpec for IFGSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifgset::R](R) reader structure"]
impl crate::Readable for IFGSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifgset::W](W) writer structure"]
impl crate::Writable for IFGSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFGset to value 0x0c"]
impl crate::Resettable for IFGSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
