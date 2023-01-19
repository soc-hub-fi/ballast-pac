#[doc = "Register `D_PIXEL_OFFSET` reader"]
pub struct R(crate::R<D_PIXEL_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_PIXEL_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_PIXEL_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_PIXEL_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_PIXEL_OFFSET` writer"]
pub struct W(crate::W<D_PIXEL_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_PIXEL_OFFSET_SPEC>;
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
impl From<crate::W<D_PIXEL_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_PIXEL_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIXEL_X_OFFSET` reader - "]
pub type PIXEL_X_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIXEL_X_OFFSET` writer - "]
pub type PIXEL_X_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_PIXEL_OFFSET_SPEC, u8, u8, 5, O>;
#[doc = "Field `PIXEL_Y_OFFSET` reader - "]
pub type PIXEL_Y_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIXEL_Y_OFFSET` writer - "]
pub type PIXEL_Y_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_PIXEL_OFFSET_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pixel_x_offset(&self) -> PIXEL_X_OFFSET_R {
        PIXEL_X_OFFSET_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pixel_y_offset(&self) -> PIXEL_Y_OFFSET_R {
        PIXEL_Y_OFFSET_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_x_offset(&mut self) -> PIXEL_X_OFFSET_W<0> {
        PIXEL_X_OFFSET_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn pixel_y_offset(&mut self) -> PIXEL_Y_OFFSET_W<16> {
        PIXEL_Y_OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For image-in mode, horizontal offset and vertical offset of the 1 st pixel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_pixel_offset](index.html) module"]
pub struct D_PIXEL_OFFSET_SPEC;
impl crate::RegisterSpec for D_PIXEL_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_pixel_offset::R](R) reader structure"]
impl crate::Readable for D_PIXEL_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_pixel_offset::W](W) writer structure"]
impl crate::Writable for D_PIXEL_OFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_PIXEL_OFFSET to value 0"]
impl crate::Resettable for D_PIXEL_OFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
