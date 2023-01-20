#[doc = "Register `D_DAIN_ADDR_LOW_1` reader"]
pub struct R(crate::R<D_DAIN_ADDR_LOW_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DAIN_ADDR_LOW_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DAIN_ADDR_LOW_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DAIN_ADDR_LOW_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DAIN_ADDR_LOW_1` writer"]
pub struct W(crate::W<D_DAIN_ADDR_LOW_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DAIN_ADDR_LOW_1_SPEC>;
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
impl From<crate::W<D_DAIN_ADDR_LOW_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DAIN_ADDR_LOW_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIN_ADDR_LOW_1` reader - "]
pub type DATAIN_ADDR_LOW_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATAIN_ADDR_LOW_1` writer - "]
pub type DATAIN_ADDR_LOW_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DAIN_ADDR_LOW_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn datain_addr_low_1(&self) -> DATAIN_ADDR_LOW_1_R {
        DATAIN_ADDR_LOW_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn datain_addr_low_1(&mut self) -> DATAIN_ADDR_LOW_1_W<0> {
        DATAIN_ADDR_LOW_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower 32bits of input data address of UV plane\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dain_addr_low_1](index.html) module"]
pub struct D_DAIN_ADDR_LOW_1_SPEC;
impl crate::RegisterSpec for D_DAIN_ADDR_LOW_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dain_addr_low_1::R](R) reader structure"]
impl crate::Readable for D_DAIN_ADDR_LOW_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dain_addr_low_1::W](W) writer structure"]
impl crate::Writable for D_DAIN_ADDR_LOW_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DAIN_ADDR_LOW_1 to value 0"]
impl crate::Resettable for D_DAIN_ADDR_LOW_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
