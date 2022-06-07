#[doc = "Register `D_BANK` reader"]
pub struct R(crate::R<D_BANK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_BANK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_BANK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_BANK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_BANK` writer"]
pub struct W(crate::W<D_BANK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_BANK_SPEC>;
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
impl From<crate::W<D_BANK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_BANK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BANK` reader - "]
pub struct DATA_BANK_R(crate::FieldReader<u8>);
impl DATA_BANK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_BANK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_BANK` writer - "]
pub struct DATA_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `WEIGHT_BANK` reader - "]
pub struct WEIGHT_BANK_R(crate::FieldReader<u8>);
impl WEIGHT_BANK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WEIGHT_BANK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEIGHT_BANK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEIGHT_BANK` writer - "]
pub struct WEIGHT_BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> WEIGHT_BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn data_bank(&self) -> DATA_BANK_R {
        DATA_BANK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn weight_bank(&self) -> WEIGHT_BANK_R {
        WEIGHT_BANK_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn data_bank(&mut self) -> DATA_BANK_W {
        DATA_BANK_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn weight_bank(&mut self) -> WEIGHT_BANK_W {
        WEIGHT_BANK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of data banks and weight banks in CBUF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_bank](index.html) module"]
pub struct D_BANK_SPEC;
impl crate::RegisterSpec for D_BANK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_bank::R](R) reader structure"]
impl crate::Readable for D_BANK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_bank::W](W) writer structure"]
impl crate::Writable for D_BANK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D_BANK to value 0"]
impl crate::Resettable for D_BANK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
