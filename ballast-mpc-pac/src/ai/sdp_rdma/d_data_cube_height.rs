#[doc = "Register `D_DATA_CUBE_HEIGHT` reader"]
pub struct R(crate::R<D_DATA_CUBE_HEIGHT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_DATA_CUBE_HEIGHT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_DATA_CUBE_HEIGHT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_DATA_CUBE_HEIGHT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D_DATA_CUBE_HEIGHT` writer"]
pub struct W(crate::W<D_DATA_CUBE_HEIGHT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_DATA_CUBE_HEIGHT_SPEC>;
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
impl From<crate::W<D_DATA_CUBE_HEIGHT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_DATA_CUBE_HEIGHT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HEIGHT` reader - "]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HEIGHT` writer - "]
pub type HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D_DATA_CUBE_HEIGHT_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<0> {
        HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d_data_cube_height](index.html) module"]
pub struct D_DATA_CUBE_HEIGHT_SPEC;
impl crate::RegisterSpec for D_DATA_CUBE_HEIGHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d_data_cube_height::R](R) reader structure"]
impl crate::Readable for D_DATA_CUBE_HEIGHT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d_data_cube_height::W](W) writer structure"]
impl crate::Writable for D_DATA_CUBE_HEIGHT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D_DATA_CUBE_HEIGHT to value 0"]
impl crate::Resettable for D_DATA_CUBE_HEIGHT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
