#[doc = "Register `PADDIR_00_31` reader"]
pub struct R(crate::R<PADDIR_00_31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADDIR_00_31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADDIR_00_31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADDIR_00_31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADDIR_00_31` writer"]
pub struct W(crate::W<PADDIR_00_31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADDIR_00_31_SPEC>;
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
impl From<crate::W<PADDIR_00_31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADDIR_00_31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - "]
pub struct DIR_R(crate::FieldReader<u32>);
impl DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - "]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit 31 - 0 DIR (R/W) GPIO\\[31:0\\]
direction configuration bitfield: - bit\\[i\\]=1b0: Input mode for GPIO\\[i\\]
- bit\\[i\\]=1b1: Output mode for GPIO\\[i\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paddir_00_31](index.html) module"]
pub struct PADDIR_00_31_SPEC;
impl crate::RegisterSpec for PADDIR_00_31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paddir_00_31::R](R) reader structure"]
impl crate::Readable for PADDIR_00_31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paddir_00_31::W](W) writer structure"]
impl crate::Writable for PADDIR_00_31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADDIR_00_31 to value 0"]
impl crate::Resettable for PADDIR_00_31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
