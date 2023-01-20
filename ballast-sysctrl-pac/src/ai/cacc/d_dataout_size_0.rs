#[doc = "Register `D_DATAOUT_SIZE_0` reader"]
pub struct R(crate::R<D_DATAOUT_SIZE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATAOUT_SIZE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATAOUT_SIZE_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATAOUT_SIZE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DATAOUT_SIZE_0` writer"]
pub struct W(crate::W<D_DATAOUT_SIZE_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DATAOUT_SIZE_0_SPEC>;
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
impl From<crate::W<D_DATAOUT_SIZE_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DATAOUT_SIZE_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAOUT_WIDTH` reader - "]
pub type DATAOUT_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATAOUT_WIDTH` writer - "]
pub type DATAOUT_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DATAOUT_SIZE_0_SPEC, u16, u16, 13, O>;
#[doc = "Field `DATAOUT_HEIGHT` reader - "]
pub type DATAOUT_HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATAOUT_HEIGHT` writer - "]
pub type DATAOUT_HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DATAOUT_SIZE_0_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn dataout_width(&self) -> DATAOUT_WIDTH_R {
        DATAOUT_WIDTH_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn dataout_height(&self) -> DATAOUT_HEIGHT_R {
        DATAOUT_HEIGHT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn dataout_width(&mut self) -> DATAOUT_WIDTH_W<0> {
        DATAOUT_WIDTH_W::new(self)
    }
    #[doc = "Bits 16:28"]
    #[inline(always)]
    #[must_use]
    pub fn dataout_height(&mut self) -> DATAOUT_HEIGHT_W<16> {
        DATAOUT_HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input cubes width and height after extension\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_dataout_size_0](index.html) module"]
pub struct D_DATAOUT_SIZE_0_SPEC;
impl crate::RegisterSpec for D_DATAOUT_SIZE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_dataout_size_0::R](R) reader structure"]
impl crate::Readable for D_DATAOUT_SIZE_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_dataout_size_0::W](W) writer structure"]
impl crate::Writable for D_DATAOUT_SIZE_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DATAOUT_SIZE_0 to value 0"]
impl crate::Resettable for D_DATAOUT_SIZE_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
